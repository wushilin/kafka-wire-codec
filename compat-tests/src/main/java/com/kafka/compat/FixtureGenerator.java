package com.kafka.compat;

import org.apache.kafka.common.Uuid;
import org.apache.kafka.common.compress.Compression;
import org.apache.kafka.common.message.ApiMessageType;
import org.apache.kafka.common.protocol.ApiMessage;
import org.apache.kafka.common.protocol.ByteBufferAccessor;
import org.apache.kafka.common.protocol.Message;
import org.apache.kafka.common.protocol.ObjectSerializationCache;
// kafka-clients 4.3.x moved the records implementation classes into the
// `record.internal` package (they remain the wire-level batch builders).
import org.apache.kafka.common.record.internal.BaseRecords;
import org.apache.kafka.common.record.internal.MemoryRecords;
import org.apache.kafka.common.record.internal.MemoryRecordsBuilder;
import org.apache.kafka.common.record.TimestampType;
import org.apache.kafka.common.utils.ImplicitLinkedHashCollection;

import java.io.RandomAccessFile;
import java.lang.reflect.Method;
import java.lang.reflect.ParameterizedType;
import java.lang.reflect.Type;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

/**
 * Comprehensive compatibility fixture generator.
 *
 * Enumerates EVERY Kafka RPC (request and response) at EVERY supported version,
 * reflectively populates every field (including nested structs, collections, and
 * tagged fields) with non-default values, and emits the serialized wire bytes.
 * For messages carrying `records`/`bytes` fields it also emits small / medium /
 * large (up to 55 MiB) payload variants.
 *
 * Output is ONE self-describing binary file:
 *
 *   magic            : 4 bytes  "KCFX"
 *   format_version   : int16    = 1
 *   record_count     : int32    (patched after writing)
 *   kafka_version    : int16 len + UTF-8 bytes
 *   records[record_count]:
 *     api_key        : int16
 *     api_version    : int16
 *     is_request     : int8  (1 = request, 0 = response)
 *     label          : int16 len + UTF-8 bytes   ("populated" | "medium" | "large")
 *     body_len       : int32
 *     body           : body_len bytes            (the encoded message body, no header)
 *
 * All integers are big-endian (DataOutput order).
 */
public class FixtureGenerator {

    static final int SMALL = 8;                 // tiny payload bytes
    static final int MEDIUM = 64 * 1024;        // 64 KiB
    static final int LARGE = 55 * 1024 * 1024;  // 55 MiB

    // Deep enough to reach e.g. FetchResponse -> topic -> partition ->
    // DivergingEpoch (depth 5); Kafka schemas are trees, so this terminates.
    static final int MAX_DEPTH = 8;
    static final int MAX_COLL_ELEMENTS = 2;

    // Only these messages get the 55 MiB variant — the canonical large-data carriers —
    // to keep the fixture file bounded.
    static final Set<String> LARGE_ALLOWLIST = new HashSet<>(Arrays.asList(
        "ProduceRequestData", "FetchResponseData"
    ));

    public static void main(String[] args) throws Exception {
        String outPath = args.length > 0 ? args[0] : "fixtures/compat.bin";
        String kafkaVersion = System.getProperty("kafka.version", "unknown");
        Path out = Paths.get(outPath);
        if (out.getParent() != null) Files.createDirectories(out.getParent());

        try (RandomAccessFile raf = new RandomAccessFile(out.toFile(), "rw")) {
            raf.setLength(0);
            raf.writeByte('K'); raf.writeByte('C'); raf.writeByte('F'); raf.writeByte('X');
            raf.writeShort(1);                  // format version
            long countPos = raf.getFilePointer();
            raf.writeInt(0);                    // placeholder record_count
            writeString(raf, kafkaVersion);

            int count = 0;
            for (ApiMessageType type : ApiMessageType.values()) {
                count += emitForType(raf, type, true);
                count += emitForType(raf, type, false);
            }

            raf.seek(countPos);
            raf.writeInt(count);
            System.out.printf("Wrote %d records to %s%n", count, out.toAbsolutePath());
        }
    }

    /** Emit all version/payload variants for one message type (request or response). */
    static int emitForType(RandomAccessFile raf, ApiMessageType type, boolean isRequest) {
        ApiMessage proto = newMessage(type, isRequest);
        if (proto == null) return 0;
        Class<?> cls = proto.getClass();

        short lo, hi;
        try {
            lo = cls.getField("LOWEST_SUPPORTED_VERSION").getShort(null);
            hi = cls.getField("HIGHEST_SUPPORTED_VERSION").getShort(null);
        } catch (Exception e) {
            return 0;
        }

        boolean payload = hasPayloadField(cls);
        boolean largeOk = payload && LARGE_ALLOWLIST.contains(cls.getSimpleName());
        int count = 0;

        for (short v = lo; v <= hi; v++) {
            Built body = buildBody(type, isRequest, v, SMALL);
            if (body == null) {
                // A version we can't serialize at all is a silent coverage hole —
                // make it loud; the Rust matrix check will fail on it anyway.
                System.err.printf("WARNING: no fixture for apikey=%d v%d %s%n",
                    type.apiKey(), v, isRequest ? "req" : "resp");
            } else {
                writeRecord(raf, type.apiKey(), v, isRequest,
                    body.populated ? "populated" : "default", body.body);
                count++;
            }

            // Null-variant: populated message with every nullable field set to
            // null. Exercises null branches (nullable strings/arrays/structs,
            // e.g. the struct presence byte) that populated records never hit.
            Built nulls = buildNullVariant(type, isRequest, v);
            if (nulls != null) {
                writeRecord(raf, type.apiKey(), v, isRequest, "nulls", nulls.body); count++;
            }

            if (payload && (v == lo || v == hi)) {
                Built med = buildBody(type, isRequest, v, MEDIUM);
                if (med != null && med.populated) {
                    writeRecord(raf, type.apiKey(), v, isRequest, "medium", med.body); count++;
                }
            }
            if (largeOk && v == hi) {
                Built big = buildBody(type, isRequest, v, LARGE);
                if (big != null && big.populated) {
                    writeRecord(raf, type.apiKey(), v, isRequest, "large", big.body); count++;
                }
            }
        }
        return count;
    }

    /**
     * Build a populated message, then null out every field (top-level and nested)
     * that tolerates null at this version, verified by trial-serialization.
     * Returns null when population failed or no field could be nulled.
     */
    static Built buildNullVariant(ApiMessageType type, boolean isRequest, short version) {
        ApiMessage msg = newMessage(type, isRequest);
        if (msg == null) return null;
        try {
            populate(msg, version, SMALL, 0);
            if (nullifyTree(msg, msg, version, 0) == 0) return null;
            return new Built(serialize(msg, version), true);
        } catch (Throwable t) {
            return null;
        }
    }

    /**
     * Recursively set nullable fields to null, keeping each null only if the
     * whole message still serializes at `version` (same trial-and-revert trick
     * as populate). Returns how many fields were nulled.
     */
    static int nullifyTree(Message node, Message root, short version, int depth) {
        if (depth > MAX_DEPTH) return 0;
        int nulled = 0;
        for (Method setter : node.getClass().getMethods()) {
            if (!setter.getName().startsWith("set") || setter.getParameterCount() != 1) continue;
            Class<?> pt = setter.getParameterTypes()[0];
            if (pt.isPrimitive()) continue;
            Object old = tryGet(node, setter.getName());
            if (old == null) continue;

            // Recurse first so nested nullables are exercised even when the
            // parent field itself cannot be nulled.
            if (old instanceof Message) {
                nulled += nullifyTree((Message) old, root, version, depth + 1);
            } else if (old instanceof Collection) {
                for (Object e : (Collection<?>) old) {
                    if (e instanceof Message) nulled += nullifyTree((Message) e, root, version, depth + 1);
                }
            }

            try {
                setter.invoke(node, new Object[]{null});
                trialSerialize(root, version);   // still valid with this field null?
                nulled++;
            } catch (Throwable t) {
                try { setter.invoke(node, old); } catch (Throwable ignored) {}
            }
        }
        return nulled;
    }

    /** A built body plus whether reflective population succeeded (vs. default fallback). */
    static final class Built {
        final byte[] body;
        final boolean populated;
        Built(byte[] body, boolean populated) { this.body = body; this.populated = populated; }
    }

    /** Build a populated message body, falling back to a default message if population fails. */
    static Built buildBody(ApiMessageType type, boolean isRequest, short version, int payloadBytes) {
        ApiMessage msg = newMessage(type, isRequest);
        if (msg == null) return null;
        try {
            populate(msg, version, payloadBytes, 0);
            return new Built(serialize(msg, version), true);
        } catch (Throwable t) {
            // Make population failures diagnosable instead of silent: these
            // become weaker "default" records, so the cause matters.
            Throwable root = t;
            while (root.getCause() != null) root = root.getCause();
            System.err.printf("populate fallback: %s %s v%d: %s: %s%n",
                msg.getClass().getSimpleName(), isRequest ? "req" : "resp", version,
                root.getClass().getSimpleName(), root.getMessage());
            ApiMessage def = newMessage(type, isRequest);
            try {
                return def == null ? null : new Built(serialize(def, version), false);
            } catch (Throwable t2) {
                return null;
            }
        }
    }

    static ApiMessage newMessage(ApiMessageType type, boolean isRequest) {
        try {
            return isRequest ? type.newRequest() : type.newResponse();
        } catch (Throwable t) {
            return null;
        }
    }

    static byte[] serialize(ApiMessage msg, short version) {
        ObjectSerializationCache cache = new ObjectSerializationCache();
        int size = msg.size(cache, version);
        ByteBuffer buf = ByteBuffer.allocate(size);
        msg.write(new ByteBufferAccessor(buf), cache, version);
        buf.flip();
        byte[] body = new byte[buf.remaining()];
        buf.get(body);
        return body;
    }

    // ── Reflective, version-safe population ────────────────────────────────────

    /**
     * Populate `node` for `version`. After setting each field we trial-serialize the
     * node at `version`; if the field is invalid for that version (Kafka throws),
     * we revert it. This keeps the message valid without needing per-field version
     * metadata, and naturally covers tagged fields (ordinary setters).
     */
    static void populate(Message node, short version, int payloadBytes, int depth) {
        if (depth > MAX_DEPTH) return;
        for (Method setter : node.getClass().getMethods()) {
            if (!setter.getName().startsWith("set")) continue;
            if (setter.getParameterCount() != 1) continue;
            // Collection elements carry intrusive linked-list indexes via
            // ImplicitLinkedHashCollection.Element#setNext/setPrev. They are
            // NOT schema fields, and setting them marks the element as
            // "already in a collection", making the later Collection.add()
            // silently return false — which left every *Collection-typed
            // (tagged) field, e.g. FetchResponse.NodeEndpoints, empty.
            if (node instanceof ImplicitLinkedHashCollection.Element
                    && (setter.getName().equals("setNext") || setter.getName().equals("setPrev"))) {
                continue;
            }

            Class<?> pt = setter.getParameterTypes()[0];
            Type gpt = setter.getGenericParameterTypes()[0];
            Object value;
            try {
                value = synth(pt, gpt, version, payloadBytes, depth);
            } catch (Throwable t) {
                debugPopulate(node, setter, version, "synth failed", t);
                continue;
            }
            if (value == null) {
                debugPopulate(node, setter, version, "synth returned null", null);
                continue;
            }

            Object old = tryGet(node, setter.getName());
            try {
                setter.invoke(node, value);
                trialSerialize(node, version);   // validity check at this version
            } catch (Throwable t) {
                debugPopulate(node, setter, version, "reverted", t);
                // Revert a field that's not valid at this version. The revert
                // must run even when the previous value was null (nullable
                // strings default to null) or the invalid value sticks.
                try { setter.invoke(node, new Object[]{old}); } catch (Throwable ignored) {}
            }
        }
    }

    /** Diagnostics for silent population gaps: set DEBUG_POPULATE=1 to enable. */
    static void debugPopulate(Message node, Method setter, short version, String what, Throwable t) {
        if (System.getenv("DEBUG_POPULATE") == null) return;
        Throwable root = t;
        while (root != null && root.getCause() != null) root = root.getCause();
        System.err.println("populate: " + node.getClass().getSimpleName() + "." + setter.getName()
            + " v" + version + " " + what + (root == null ? "" : (": " + root)));
    }

    /**
     * Full size + write trial: some per-version validity checks (e.g.
     * "Attempted to write a non-default X at version N") only fire in one of
     * the two, so sizing alone lets invalid values slip through to the final
     * serialization.
     */
    static void trialSerialize(Message node, short version) {
        ObjectSerializationCache cache = new ObjectSerializationCache();
        int size = node.size(cache, version);
        node.write(new ByteBufferAccessor(ByteBuffer.allocate(size)), cache, version);
    }

    static Object tryGet(Message node, String setterName) {
        String prop = setterName.substring(3);
        String getter = Character.toLowerCase(prop.charAt(0)) + prop.substring(1);
        try {
            Method g = node.getClass().getMethod(getter);
            return g.invoke(node);
        } catch (Throwable t) {
            return null;
        }
    }

    /** Synthesize a non-default value for a field of the given type. */
    @SuppressWarnings("unchecked")
    static Object synth(Class<?> pt, Type gpt, short version, int payloadBytes, int depth) throws Exception {
        if (pt == boolean.class || pt == Boolean.class) return Boolean.TRUE;
        if (pt == byte.class || pt == Byte.class) return (byte) 7;
        if (pt == short.class || pt == Short.class) return (short) 7;
        if (pt == int.class || pt == Integer.class) return 7;
        if (pt == long.class || pt == Long.class) return 7L;
        if (pt == double.class || pt == Double.class) return 1.5d;
        if (pt == float.class || pt == Float.class) return 1.5f;
        if (pt == String.class) return "s";
        if (pt == Uuid.class) return new Uuid(7L, 13L);
        if (pt == byte[].class) return makeBytes(payloadBytes);
        if (pt == ByteBuffer.class) return ByteBuffer.wrap(makeBytes(payloadBytes));
        if (BaseRecords.class.isAssignableFrom(pt)) return makeRecords(payloadBytes);

        // Nested struct: construct and fully populate it now (so collection
        // elements are complete before being added — preserves hash invariants).
        if (Message.class.isAssignableFrom(pt)) {
            Message nested = (Message) pt.getDeclaredConstructor().newInstance();
            populate(nested, version, payloadBytes, depth + 1);
            return nested;
        }

        // java.util.List<T>
        if (List.class.isAssignableFrom(pt)) {
            Class<?> elem = elemTypeOf(gpt);
            if (elem == null) return null;
            List<Object> list = new ArrayList<>();
            int n = Message.class.isAssignableFrom(elem) ? 1 : MAX_COLL_ELEMENTS;
            for (int i = 0; i < n; i++) {
                Object e = synth(elem, elem, version, payloadBytes, depth + 1);
                if (e != null) list.add(e);
            }
            return list;
        }

        // Kafka *Collection (ImplicitLinkedHash{,Multi}Collection<T>)
        if (Collection.class.isAssignableFrom(pt)) {
            Object coll = pt.getDeclaredConstructor().newInstance();
            Class<?> elem = elemTypeOfSuper(pt);
            if (elem != null) {
                Object e = synth(elem, elem, version, payloadBytes, depth + 1);
                if (e != null) ((Collection<Object>) coll).add(e);
            }
            return coll;
        }

        return null; // unknown type — leave default
    }

    static Class<?> elemTypeOf(Type gpt) {
        if (gpt instanceof ParameterizedType) {
            Type[] args = ((ParameterizedType) gpt).getActualTypeArguments();
            if (args.length == 1 && args[0] instanceof Class) return (Class<?>) args[0];
        }
        return null;
    }

    static Class<?> elemTypeOfSuper(Class<?> collClass) {
        Type sup = collClass.getGenericSuperclass();
        Class<?> e = elemTypeOf(sup);
        if (e != null) return e;
        // Walk up one more level if needed.
        if (collClass.getSuperclass() != null) return elemTypeOfSuper(collClass.getSuperclass());
        return null;
    }

    // ── Payload builders ───────────────────────────────────────────────────────

    static byte[] makeBytes(int n) {
        byte[] b = new byte[Math.max(n, 0)];
        Arrays.fill(b, (byte) 0x61);
        return b;
    }

    static MemoryRecords makeRecords(int targetBytes) {
        int cap = Math.max(targetBytes + 1024, 1024);
        ByteBuffer buf = ByteBuffer.allocate(cap);
        MemoryRecordsBuilder builder = MemoryRecords.builder(
            buf, Compression.NONE, TimestampType.CREATE_TIME, 0L);
        int perRecord = Math.min(Math.max(targetBytes, 16), 1 << 20); // cap 1 MiB/record
        byte[] value = new byte[perRecord];
        Arrays.fill(value, (byte) 0x62);
        long offset = 0;
        do {
            builder.appendWithOffset(offset++, 0L, null, value);
        } while (buf.position() < targetBytes && offset < 2_000_000);
        return builder.build();
    }

    static boolean hasPayloadField(Class<?> cls) {
        return hasPayloadField(cls, new HashSet<>(), 0);
    }

    /** Recursively check whether a message (or any nested struct) carries a bytes/records field. */
    static boolean hasPayloadField(Class<?> cls, Set<Class<?>> seen, int depth) {
        if (depth > MAX_DEPTH || !seen.add(cls)) return false;
        for (Method m : cls.getMethods()) {
            if (!m.getName().startsWith("set") || m.getParameterCount() != 1) continue;
            Class<?> pt = m.getParameterTypes()[0];
            if (pt == byte[].class || pt == ByteBuffer.class || BaseRecords.class.isAssignableFrom(pt)) {
                return true;
            }
            if (Message.class.isAssignableFrom(pt)) {
                if (hasPayloadField(pt, seen, depth + 1)) return true;
            } else if (Collection.class.isAssignableFrom(pt) || List.class.isAssignableFrom(pt)) {
                Class<?> elem = List.class.isAssignableFrom(pt)
                    ? elemTypeOf(m.getGenericParameterTypes()[0])
                    : elemTypeOfSuper(pt);
                if (elem != null && Message.class.isAssignableFrom(elem)
                    && hasPayloadField(elem, seen, depth + 1)) {
                    return true;
                }
            }
        }
        return false;
    }

    // ── File output ────────────────────────────────────────────────────────────

    static void writeRecord(RandomAccessFile raf, short apiKey, short version,
                             boolean isRequest, String label, byte[] body) {
        try {
            raf.writeShort(apiKey);
            raf.writeShort(version);
            raf.writeByte(isRequest ? 1 : 0);
            writeString(raf, label);
            raf.writeInt(body.length);
            raf.write(body);
        } catch (Exception e) {
            throw new RuntimeException("write failed", e);
        }
    }

    static void writeString(RandomAccessFile raf, String s) throws Exception {
        byte[] b = s.getBytes(StandardCharsets.UTF_8);
        raf.writeShort(b.length);
        raf.write(b);
    }
}
