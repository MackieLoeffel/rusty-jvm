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
        i += -10;

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

    public void reference() {
        Object a = null;
        Object b = a;
    }

    public void array() {
        boolean[] a = new boolean[2];
        a[0] = a[1];
        byte[] b = new byte[2];
        b[0] = b[1];
        short[] c = new short[2];
        c[0] = c[1];
        int[] i = new int[2];
        i[0] = i[1];
        long[] d = new long[2];
        d[0] = d[1];
        float[] e = new float[2];
        e[0] = e[1];
        double[] f = new double[2];
        f[0] = f[1];
        char[] g = new char[2];
        g[0] = g[1];
        Object[] h = new Object[2];
        h[0] = h[1];
        Object[][] j = new Object[2][2];
        j[0][0] = j[1][1];
    }
}
