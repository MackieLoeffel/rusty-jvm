package com.mackie.rustyjvm;

public class TestVM {
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
}
