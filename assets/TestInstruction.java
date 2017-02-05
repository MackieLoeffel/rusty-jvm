package com.mackie.rustyjvm;

public class TestInstruction {
    void conversions() {
        int a = 1;
        byte b = (byte) a;
        short c = (short) a;
        long d = (long) a;
        float e = (float) a;
        double f = (double) a;
        a = (int) d;
        e = (float) d;
        f = (double) d;
        a = (int) e;
        d = (long) e;
        f = (double) e;
        a = (int) f;
        d = (long) f;
        e = (float) f;
    }

    void arithmetic() {
        int i = 1;
        long l = 1;
        float f = 1;
        double d = 1;
        i = i + 1;
        i = i - 1;
        i = i * 1;
        i = i / 1;
        i = i % 1;
        i = -i;
        i = i << 1;
        i = i >> 1;
        i = i >>> 1;
        i = i & 1;
        i = i | 1;
        i = i ^ 1;

        l = l + 1;
        l = l - 1;
        l = l * 1;
        l = l / 1;
        l = l % 1;
        l = -l;
        l = l << 1;
        l = l >> 1;
        l = l >>> 1;
        l = l & 1;
        l = l | 1;
        l = l ^ 1;

        f = f + 1;
        f = f - 1;
        f = f * 1;
        f = f / 1;
        f = f % 1;
        f = -f;

        d = d + 1;
        d = d - 1;
        d = d * 1;
        d = d / 1;
        d = d % 1;
        d = -d;
    }
}
