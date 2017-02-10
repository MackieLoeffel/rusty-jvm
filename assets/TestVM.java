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

    private static void mul() {
        int a = 4;
        nativeInt(2 * a);
        nativeInt(0x40000001 * a);
        long l = 4L;
        nativeLong(2L * l);
        nativeLong(0x40000001L * l * 4);
        nativeLong(0x4000000000000001L * l);
        float f = 0.1f;
        nativeFloat(f * 2f);
        double d = 0.1;
        nativeDouble(d * 2);
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
