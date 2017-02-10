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
        int a = 1;
        a = staticMethod(a);
        nativeInt(a);
    }

    private static int staticMethod(int a) {
        nativeInt(a);
        a = a * 2;
        nativeInt(a);
        return a;
    }

    private static void mul() {
        nativeInt(2 * 2);
        nativeInt(0x40000001 * 4);
        // TODO rest
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
