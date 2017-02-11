package com.mackie.rustyjvm;

public class TestVM {
    public static native void nativeBoolean(boolean i);
    public static native void nativeByte(byte i);
    public static native void nativeShort(short i);
    public static native void nativeInt(int i);
    public static native void nativeLong(long i);
    public static native void nativeDouble(double i);
    public static native void nativeFloat(float i);
    public static native void nativeString(String s);

    public static void simple() {
        int a = 1;
        nativeInt(a);
    }

    public static void staticcall() {
        long a = 1;
        a = staticMethod(a);
        nativeLong(a);
    }

    private static long staticMethod(long a) {
        nativeLong(a);
        a = a * 2;
        nativeLong(a);
        return a;
    }

    private static void add() {
        int a = 4;
        // first is constant folded by the compiler
        // second is calculated by us
        nativeInt(2 + 4);
        nativeInt(2 + a);
        a = 0x7FFFFFFF;
        nativeInt(0x7FFFFFFF + 0x7FFFFFFF);
        nativeInt(0x7FFFFFFF + a);
        nativeInt(-1 + 0x7FFFFFFF);
        nativeInt(-1 + a);
        long l = 4L;
        nativeLong(2L + 4L);
        nativeLong(2L + l);
        l = 0x7FFFFFFFL;
        nativeLong(0x7FFFFFFFL + 0x7FFFFFFFL);
        nativeLong(0x7FFFFFFFL + l);
        l = 0x7FFFFFFFFFFFFFFFL;
        nativeLong(0x7FFFFFFFFFFFFFFFL + 0x7FFFFFFFFFFFFFFFL);
        nativeLong(0x7FFFFFFFFFFFFFFFL + l);
        nativeLong(-1 + 0x7FFFFFFFFFFFFFFFL);
        nativeLong(-1 + l);
        float f = 0.1f;
        nativeFloat(0.1f + 2f);
        nativeFloat(f + 2f);
        double d = 0.1;
        nativeDouble(0.1 + 2);
        nativeDouble(d + 2);
        // TODO Test starnger float numbers?
    }

    private static void sub() {
        int a = 4;
        nativeInt(2 - 4);
        nativeInt(2 - a);
        a = 0x7FFFFFFF;
        nativeInt(0x80000000 - 0x7FFFFFFF);
        nativeInt(0x80000000 - a);
        long l = 4L;
        nativeLong(2L - 4L);
        nativeLong(2L - l);
        l = 0x7FFFFFFFL;
        nativeLong(0x80000000L - 0x7FFFFFFFL);
        nativeLong(0x80000000L - l);
        l = 0x7FFFFFFFFFFFFFFFL;
        nativeLong(0x8000000000000000L - 0x7FFFFFFFFFFFFFFFL);
        nativeLong(0x8000000000000000L - l);
        float f = 0.1f;
        nativeFloat(0.1f - 2f);
        nativeFloat(f - 2f);
        double d = 0.1;
        nativeDouble(0.1 - 2);
        nativeDouble(d - 2);
        // TODO Test starnger float numbers?
    }

    private static void mul() {
        int a = 4;
        nativeInt(2 * 4);
        nativeInt(2 * a);
        nativeInt(0x40000001 * 4);
        nativeInt(0x40000001 * a);
        long l = 4L;
        nativeLong(2L * 4L);
        nativeLong(2L * l);
        nativeLong(0x40000001L * 4L * 4);
        nativeLong(0x40000001L * l * 4);
        nativeLong(0x4000000000000001L * 4L);
        nativeLong(0x4000000000000001L * l);
        float f = 0.1f;
        nativeFloat(0.1f * 2f);
        nativeFloat(f * 2f);
        double d = 0.1;
        nativeDouble(0.1 * 2);
        nativeDouble(d * 2);
        // TODO Test starnger float numbers?
    }

    private static void div() {
        int a = 4;
        nativeInt(6 / 4);
        nativeInt(6 / a);
        nativeInt(-6 / 4);
        nativeInt(-6 / a);
        a = -1;
        nativeInt(0x80000000 / -1);
        nativeInt(0x80000000 / a);
        // TODO test divide by 0
        long l = 4L;
        nativeLong(6L / 4L);
        nativeLong(6L / l);
        nativeLong(-6L / 4L);
        nativeLong(-6L / l);
        l = -1;
        nativeLong(0x8000000000000000L / -1L);
        nativeLong(0x8000000000000000L / l);
        // TODO test divide by 0

        float f = 0.1f;
        nativeFloat(0.1f / 2f);
        nativeFloat(f / 2f);
        double d = 0.1;
        nativeDouble(0.1 / 2);
        nativeDouble(d / 2);
        // TODO Test starnger float numbers?
    }

    private static void rem() {
        int a = 4;
        nativeInt(6 % 4);
        nativeInt(6 % a);
        nativeInt(-6 % 4);
        nativeInt(-6 % a);
        a = -1;
        nativeInt(0x80000000 % -1);
        nativeInt(0x80000000 % a);
        // TODO test divide by 0
        long l = 4L;
        nativeLong(6L % 4L);
        nativeLong(6L % l);
        nativeLong(-6L % 4L);
        nativeLong(-6L % l);
        l = -1;
        nativeLong(0x8000000000000000L % -1L);
        nativeLong(0x8000000000000000L % l);
        // TODO test divide by 0

        float f = 2.1f;
        nativeFloat(2.1f % 2f);
        nativeFloat(f % 2f);
        double d = 2.1;
        nativeDouble(2.1 % 2);
        nativeDouble(d % 2);
        // TODO Test starnger float numbers?
    }

    private static void neg() {
        int a = 4;
        nativeInt(-4);
        nativeInt(-a);
        a = -1;
        nativeInt(-(-1));
        nativeInt(-a);
        a = 0x80000000;
        nativeInt(-(0x80000000));
        nativeInt(-a);
        long l = 4L;
        nativeLong(-4L);
        nativeLong(-l);
        l = -1L;
        nativeLong(-(-1L));
        nativeLong(-l);
        l = 0x8000000000000000L;
        nativeLong(-(0x8000000000000000L));
        nativeLong(-l);
        float f = 0.1f;
        nativeFloat(-0.1f);
        nativeFloat(-f);
        double d = 0.1;
        nativeDouble(-0.1);
        nativeDouble(-d);
        // TODO Test starnger float numbers?
    }

    private static void shift() {
        // shift left
        int a = 0xF;
        nativeInt(0xF << 4);
        nativeInt(a << 4);
        nativeInt(0xF << 33);
        nativeInt(a << 33);
        a = 1;
        nativeInt(1 << 31);
        nativeInt(a << 31);
        a = 0x80000000;
        nativeInt(0x80000000 << 1);
        nativeInt(a << 1);
        long l = 0xFL;
        nativeLong(0xFL << 4);
        nativeLong(l << 4);
        nativeLong(0xFL << 65);
        nativeLong(l << 65);
        l = 1;
        nativeLong(1L << 63);
        nativeLong(l << 63);
        l = 0x8000000000000000L;
        nativeLong(0x8000000000000000L << 1);
        nativeLong(l << 1);

        // shift right
        a = 0xFF;
        nativeInt(0xFF >> 4);
        nativeInt(a >> 4);
        nativeInt(0xFF >> 33);
        nativeInt(a >> 33);
        a = 0x80000000;
        nativeInt(0x80000000 >> 1);
        nativeInt(a >> 1);
        a = -1;
        nativeInt(-1 >> 1);
        nativeInt(a >> 1);
        l = 0xFFL;
        nativeLong(0xFFL >> 4);
        nativeLong(l >> 4);
        nativeLong(0xFFL >> 65);
        nativeLong(l >> 65);
        l = 0x8000000000000000L;
        nativeLong(0x8000000000000000L >> 1);
        nativeLong(l >> 1);
        l = -1;
        nativeLong(-1 >> 1);
        nativeLong(l >> 1);

        // unsigned shift right
        a = 0xFF;
        nativeInt(0xFF >>> 4);
        nativeInt(a >>> 4);
        nativeInt(0xFF >>> 33);
        nativeInt(a >>> 33);
        a = 0x80000000;
        nativeInt(0x80000000 >>> 1);
        nativeInt(a >>> 1);
        a = -1;
        nativeInt(-1 >>> 1);
        nativeInt(a >>> 1);
        l = 0xFFL;
        nativeLong(0xFFL >>> 4);
        nativeLong(l >>> 4);
        nativeLong(0xFFL >>> 65);
        nativeLong(l >>> 65);
        l = 0x8000000000000000L;
        nativeLong(0x8000000000000000L >>> 1);
        nativeLong(l >>> 1);
        l = -1;
        nativeLong(-1L >>> 1);
        nativeLong(l >>> 1);
    }

    private static void bitops() {
        int a = 12; // 0b1100
        nativeInt(12 & 10); // 0b1010
        nativeInt(a & 10);
        nativeInt(12 | 10);
        nativeInt(a | 10);
        nativeInt(12 ^ 10);
        nativeInt(a ^ 10);
        long l = 12L;
        nativeLong(12L & 10L);
        nativeLong(l & 10L);
        nativeLong(12L | 10L);
        nativeLong(l | 10L);
        nativeLong(12L ^ 10L);
        nativeLong(l ^ 10L);
    }

    private static void iinc() {
        int a = 0x7FFFFFFF;
        a += 1;
        nativeInt(a);
        a -= 1;
        nativeInt(a);
        a += -15;
        nativeInt(a);
    }

    private static void constants() {
        nativeInt(0);
        nativeInt(1337);
        nativeInt(0x4000000);
        nativeFloat(0f);
        nativeFloat(1f);
        nativeFloat(2f);
        nativeFloat(1.337f);
        nativeDouble(0);
        nativeDouble(1);
        nativeDouble(1.337);
        nativeLong(0L);
        nativeLong(1L);
        nativeLong(1337L);
        nativeString(null);
        // TODO test constant string
    }

    private static void conversions() {
        int a = 0x1FF;
        nativeByte((byte) 0x1FF);
        nativeByte((byte) a);
        a = 0x1FFFF;
        nativeShort((short) 0x1FFFF);
        nativeShort((short) a);

        // TODO test more numbers (NaN, inf,...)
        a = 5;
        nativeLong((long) 5);
        nativeLong((long) a);
        nativeFloat((float) 5);
        nativeFloat((float) a);
        nativeDouble((double) 5);
        nativeDouble((double) a);

        long l = 0x100000001L;
        nativeInt((int) 0x100000001L);
        nativeInt((int) l);
        nativeFloat((float) 0x100000001L);
        nativeFloat((float) l);
        nativeDouble((double) 0x100000001L);
        nativeDouble((double) l);

        float f = -2.1f;
        nativeInt((int) -2.1f);
        nativeInt((int) f);
        nativeLong((long) -2.1f);
        nativeLong((long) f);
        nativeDouble((double) -2.1f);
        nativeDouble((double) f);

        double d = -2.1;
        nativeInt((int) -2.1);
        nativeInt((int) d);
        nativeLong((long) -2.1);
        nativeLong((long) d);
        nativeFloat((float) -2.1);
        nativeFloat((float) d);
    }

    public static void jumps() {
        for(int i = 0; i < 2; i++) {
            nativeInt(-10 + i);
        }
        int i = 1;
        Object o = null;

        if(i < 1) { nativeInt(0); }
        if(i <= 1) { nativeInt(1); }
        if(i == 1) { nativeInt(2); }
        if(i != 1) { nativeInt(3); }
        if(i >= 1) { nativeInt(4); }
        if(i > 1) { nativeInt(5); }

        if(i < 0) { nativeInt(6); }
        if(i <= 0) { nativeInt(7); }
        if(i == 0) { nativeInt(8); }
        if(i != 0) { nativeInt(9); }
        if(i >= 0) { nativeInt(10); }
        if(i > 0) { nativeInt(11); }

        if(o == o) { nativeInt(12); }
        if(o != o) { nativeInt(13); }
        if(o == null) { nativeInt(14); }
        if(o != null) { nativeInt(15); }

        float f = 0.9f;
        double d = 1.1;
        long l = 1;
        nativeBoolean(d < 1.0);
        nativeBoolean(d > 1.0);
        nativeBoolean(f < 1.0f);
        nativeBoolean(f > 1.0f);
        nativeBoolean(l == 1);
        nativeBoolean(l > 1);
        nativeBoolean(l < 1);

        d = Double.NaN;
        f = Float.NaN;
        nativeBoolean(d < 1.0);
        nativeBoolean(d > 1.0);
        nativeBoolean(f < 1.0f);
        nativeBoolean(f > 1.0f);
    }
}
