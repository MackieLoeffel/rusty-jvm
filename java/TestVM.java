package com.mackie.rustyjvm;

interface TestVMInterfaceA {}
interface TestVMInterfaceB extends TestVMInterfaceA {}
interface TestVMInterfaceC extends TestVMInterfaceA {}
interface TestVMInterfaceD extends TestVMInterfaceB, TestVMInterfaceC {}

class TestVMSuper implements TestVMInterfaceA {
    protected long superLong;
    protected int superInt;
    public TestVMSuper(int intv) {
        superInt = intv;
    }

    public long virtualMethod(long a) {
        TestVM.nativeInt(200);
        TestVM.nativeLong(a);
        return a + 100;
    }
    public static double staticMethod(double a) {
        TestVM.nativeInt(2);
        TestVM.nativeDouble(a);
        a = a * 3;
        return a;
    }

    private void privateMethod() {
        TestVM.nativeInt(1337);
    }
}

class TestVMOtherSuper implements TestVMInterfaceB {
    public long virtualMethod(long a) {
        TestVM.nativeInt(2000);
        TestVM.nativeLong(a);
        return a + 100;
    }
    public static double staticMethod(double a) {
        TestVM.nativeInt(3);
        TestVM.nativeDouble(a);
        a = a * 4;
        return a;
    }
}

class TestVMOther extends TestVMOtherSuper {
    public long virtualMethod(long a) {
        TestVM.nativeInt(1000);
        a = super.virtualMethod(a + 10);
        TestVM.nativeInt(3000);
        TestVM.nativeLong(a);
        return a;
    }

    public static double staticMethod(double a) {
        TestVM.nativeInt(4);
        TestVM.nativeDouble(a);
        a = a * 5;
        return a;
    }
}

public class TestVM extends TestVMSuper implements TestVMInterfaceD {
    public static native void nativeBoolean(boolean i);
    public static native void nativeChar(char i);
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

    public static void invoke() {
        TestVM vm = new TestVM(1, 1);
        TestVMSuper vmSuper = vm;
        TestVMSuper vmSuperReal = new TestVMSuper(2);
        TestVMOther other = new TestVMOther();
        TestVMOtherSuper otherSuper = other;
        TestVMOtherSuper otherSuperReal = new TestVMOtherSuper();
        nativeLong(vm.virtualMethod(1));
        nativeLong(vmSuper.virtualMethod(1));
        nativeLong(vmSuperReal.virtualMethod(1));
        nativeLong(other.virtualMethod(1));
        nativeLong(otherSuper.virtualMethod(1));
        nativeLong(otherSuperReal.virtualMethod(1));

        nativeDouble(vm.staticMethod(1));
        nativeDouble(vmSuper.staticMethod(1));
        nativeDouble(vmSuperReal.staticMethod(1));
        nativeDouble(other.staticMethod(1));
        nativeDouble(otherSuper.staticMethod(1));
        nativeDouble(otherSuperReal.staticMethod(1));

        vm.privateMethod();
    }

    public static double staticMethod(double a) {
        nativeInt(1);
        nativeDouble(a);
        a = a * 2;
        return a;
    }

    public long virtualMethod(long a) {
        nativeInt(100);
        a = super.virtualMethod(a + 10);
        nativeInt(300);
        nativeLong(a);
        return a;
    }

    private void privateMethod() {
        nativeInt(42);
    }

    private static void castinstanceof() {
        Object vm = new TestVM(1, 1);
        Object vmSuperReal = new TestVMSuper(2);
        Object other = new TestVMOther();
        Object nullObject = null;
        Object array = new TestVMSuper[0];
        Object intArray = new int[2];

        nativeBoolean(vm instanceof TestVM);
        nativeBoolean(vm instanceof TestVMSuper);
        nativeBoolean(vm instanceof TestVMOther);
        nativeBoolean(vm instanceof Object);
        nativeBoolean(vm instanceof TestVMInterfaceA);
        nativeBoolean(vm instanceof TestVMInterfaceB);
        nativeBoolean(vm instanceof TestVMInterfaceC);
        nativeBoolean(vm instanceof TestVMInterfaceD);

        nativeBoolean(vmSuperReal instanceof TestVM);
        nativeBoolean(vmSuperReal instanceof TestVMSuper);
        nativeBoolean(vmSuperReal instanceof TestVMOther);
        nativeBoolean(vmSuperReal instanceof Object);
        nativeBoolean(vmSuperReal instanceof TestVMInterfaceA);
        nativeBoolean(vmSuperReal instanceof TestVMInterfaceB);
        nativeBoolean(vmSuperReal instanceof TestVMInterfaceC);
        nativeBoolean(vmSuperReal instanceof TestVMInterfaceD);

        nativeBoolean(other instanceof TestVMOther);
        nativeBoolean(other instanceof Object);
        nativeBoolean(other instanceof TestVMInterfaceA);
        nativeBoolean(other instanceof TestVMInterfaceB);
        nativeBoolean(other instanceof TestVMInterfaceC);
        nativeBoolean(other instanceof TestVMInterfaceD);

        nativeBoolean(nullObject instanceof TestVM);
        nativeBoolean(nullObject instanceof TestVMSuper);
        nativeBoolean(nullObject instanceof TestVMOther);
        nativeBoolean(nullObject instanceof Object);
        nativeBoolean(nullObject instanceof String);
        nativeBoolean(nullObject instanceof TestVMInterfaceA);
        nativeBoolean(nullObject instanceof TestVMInterfaceB);
        nativeBoolean(nullObject instanceof TestVMInterfaceC);
        nativeBoolean(nullObject instanceof TestVMInterfaceD);

        nativeBoolean(array instanceof TestVM[]);
        nativeBoolean(array instanceof TestVMSuper[]);
        nativeBoolean(array instanceof TestVMOther[]);
        nativeBoolean(array instanceof Object[]);
        nativeBoolean(array instanceof String[]);
        nativeBoolean(array instanceof TestVMInterfaceA[]);
        nativeBoolean(array instanceof TestVMInterfaceB[]);
        nativeBoolean(array instanceof TestVMInterfaceC[]);
        nativeBoolean(array instanceof TestVMInterfaceD[]);
        nativeBoolean(array instanceof Cloneable);
        nativeBoolean(array instanceof java.io.Serializable);
        nativeBoolean(array instanceof TestVMInterfaceA);

        nativeBoolean(intArray instanceof int[]);
        nativeBoolean(intArray instanceof double[]);

        vmSuperReal = (TestVMSuper) vm;
        vmSuperReal = (TestVMSuper) null;
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
        a = 0x1FFFF;
        nativeChar((char) 0x1FFFF);
        nativeChar((char) a);

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

    private static void arrays() {
        long[][] l = new long[2][3];
        int[][] i = new int[][] {{1}};
        long[][] l2 = new long[2][2];
        int[][] i2 = new int[][] {{1}};
        long[] l3 = new long[] {1, 2};
        nativeLong(l[0][1]);
        l[0][1] = 5;
        nativeLong(l[0][1]);
        nativeInt(i[0][0]);
        i[0][0] = 2;
        nativeInt(i[0][0]);
        nativeLong(l2[0][1]);
        l2[0] = l[0];
        nativeLong(l2[0][1]);
        nativeInt(i2[0][0]);
        i2[0] = i[0];
        nativeInt(i2[0][0]);

        nativeLong(l3[0]);
        nativeLong(l3[1]);

        nativeInt(l2.length);
        nativeInt(l2[0].length);
        nativeInt(l2[1].length);
    }

    private int intField;
    private long longField = 2;
    private double doubleField;
    public TestVM(int a, int b) {
        super(b);
        intField = a;
        doubleField = a * 2;
    }

    private static void object() {
        TestVM a = new TestVM(10, 50);
        nativeInt(a.intField);
        nativeLong(a.longField);
        nativeDouble(a.doubleField);
        nativeLong(a.superLong);
        nativeInt(a.superInt);

        a.intField = 20;
        nativeInt(a.intField);
        nativeLong(a.longField);
        nativeDouble(a.doubleField);
        nativeLong(a.superLong);
        nativeInt(a.superInt);

        a.longField += 22;
        nativeInt(a.intField);
        nativeLong(a.longField);
        nativeDouble(a.doubleField);
        nativeLong(a.superLong);
        nativeInt(a.superInt);

        a.doubleField *= 2;
        nativeInt(a.intField);
        nativeLong(a.longField);
        nativeDouble(a.doubleField);
        nativeLong(a.superLong);
        nativeInt(a.superInt);

        a.superLong += 2;
        nativeInt(a.intField);
        nativeLong(a.longField);
        nativeDouble(a.doubleField);
        nativeLong(a.superLong);
        nativeInt(a.superInt);

        a.superInt = 200;
        nativeInt(a.intField);
        nativeLong(a.longField);
        nativeDouble(a.doubleField);
        nativeLong(a.superLong);
        nativeInt(a.superInt);
    }
}
