package org.example;

import org.graalvm.polyglot.Context;
import org.graalvm.polyglot.Source;
import org.graalvm.polyglot.Value;
import org.graalvm.polyglot.io.ByteSequence;

import java.io.IOException;

public class Main {

    public static void main(String[] args) throws IOException {
        byte[] wasmBytes = Main.class.getResourceAsStream("/fibonacci.wasm").readAllBytes();

        try (Context context = Context.newBuilder("wasm")
                .option("wasm.Builtins", "wasi_snapshot_preview1")
                .allowExperimentalOptions(true)
                .build()) {

            Source source = Source.newBuilder("wasm",
                            ByteSequence.create(wasmBytes),
                            "fibonacci")
                    .build();

            Value module   = context.eval(source);
            Value instance = module.newInstance();
            Value exports  = instance.getMember("exports");

            int n = 10;

            Value memory = exports.getMember("memory");
            int   ptr    = exports.invokeMember("fibonacci", n).asInt();

            int len = readInt(memory, ptr);

            int[] sequence = new int[len];
            for (int i = 0; i < len; i++) {
                sequence[i] = readInt(memory, ptr + 4 + i * 4);
            }

            exports.invokeMember("free_fibonacci", ptr, n);

            System.out.println("Fibonacci(" + n + "): " + java.util.Arrays.toString(sequence));
        }
    }

    /** Read a little-endian i32 from WASM linear memory at the given byte offset. */
    private static int readInt(Value memory, int byteOffset) {
        int b0 = memory.readBufferByte(byteOffset)     & 0xFF;
        int b1 = memory.readBufferByte(byteOffset + 1) & 0xFF;
        int b2 = memory.readBufferByte(byteOffset + 2) & 0xFF;
        int b3 = memory.readBufferByte(byteOffset + 3) & 0xFF;
        return b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);
    }
}