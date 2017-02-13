

public class Jump {
    private static void dump_char(char c) {
        System.out.print(c);
    }
    private static void dump_char2(char c1, char c2) {
        dump_char(c1);
        dump_char(c2);
    }
    private static void dump_char3(char c1, char c2, char c3) {
        dump_char(c1);
        dump_char(c2);
        dump_char(c3);
    }
    private static void dump_char4(char c1, char c2, char c3, char c4) {
        dump_char(c1);
        dump_char(c2);
        dump_char(c3);
        dump_char(c4);
    }

    private static void dump_long_rec(long x) {
        if (x == 0) { return; }
        dump_long_rec(x / 10);
        dump_char((char)('0' + (x % 10)));
    }
    private static void dump_long(long x) {
        if (x == 0) {
            dump_char('0');
            return;
        }

        // special handling for minimal long value
        // because -Long.MIN_VALUE == Long.MIN_VALUE
        if(x == Long.MIN_VALUE) {
            dump_char2('-', '9');
            x = 223372036854775808L;
        }

        if(x < 0) {
            dump_char('-');
            x = -x;
        }
        dump_long_rec(x);
    }

    private static void test_byte(byte a, byte b) {
        if (a < b) {
            dump_long(a);
            dump_char3(' ', '<', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a <= b) {
            dump_long(a);
            dump_char4(' ', '<', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a > b) {
            dump_long(a);
            dump_char3(' ', '>', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a >= b) {
            dump_long(a);
            dump_char4(' ', '>', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a == b) {
            dump_long(a);
            dump_char4(' ', '=', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a != b) {
            dump_long(a);
            dump_char4(' ', '!', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
    }

    private static void test_short(short a, short b) {
        if (a < b) {
            dump_long(a);
            dump_char3(' ', '<', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a <= b) {
            dump_long(a);
            dump_char4(' ', '<', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a > b) {
            dump_long(a);
            dump_char3(' ', '>', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a >= b) {
            dump_long(a);
            dump_char4(' ', '>', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a == b) {
            dump_long(a);
            dump_char4(' ', '=', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a != b) {
            dump_long(a);
            dump_char4(' ', '!', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
    }

    private static void test_int(int a, int b) {
        if (a < b) {
            dump_long(a);
            dump_char3(' ', '<', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a <= b) {
            dump_long(a);
            dump_char4(' ', '<', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a > b) {
            dump_long(a);
            dump_char3(' ', '>', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a >= b) {
            dump_long(a);
            dump_char4(' ', '>', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a == b) {
            dump_long(a);
            dump_char4(' ', '=', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a != b) {
            dump_long(a);
            dump_char4(' ', '!', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
    }

    private static void test_long(long a, long b) {
        if (a < b) {
            dump_long(a);
            dump_char3(' ', '<', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a <= b) {
            dump_long(a);
            dump_char4(' ', '<', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a > b) {
            dump_long(a);
            dump_char3(' ', '>', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a >= b) {
            dump_long(a);
            dump_char4(' ', '>', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a == b) {
            dump_long(a);
            dump_char4(' ', '=', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
        if (a != b) {
            dump_long(a);
            dump_char4(' ', '!', '=', ' ');
            dump_long(b);
            dump_char('\n');
        }
    }

    public static void main(String[] args) {
        long val[] = {
            Long.MIN_VALUE, Long.MIN_VALUE + 1, Long.MIN_VALUE + 2,
            Integer.MIN_VALUE, Integer.MIN_VALUE + 1, Integer.MIN_VALUE + 2,
            Short.MIN_VALUE, Short.MIN_VALUE + 1, Short.MIN_VALUE + 2,
            Byte.MIN_VALUE, Byte.MIN_VALUE + 1, Byte.MIN_VALUE + 2,
            -2, -1,
            0,
            1, 2,
            Byte.MAX_VALUE - 2, Byte.MAX_VALUE - 1, Byte.MAX_VALUE,
            Short.MAX_VALUE - 2, Short.MAX_VALUE - 1, Short.MAX_VALUE,
            Integer.MAX_VALUE - 2, Integer.MAX_VALUE - 1, Integer.MAX_VALUE,
            Long.MAX_VALUE - 2, Long.MAX_VALUE - 1, Long.MAX_VALUE,
        };
        int i;
        int j;
        dump_char3('v', '0', ':');
        dump_long(val[0]);
        dump_char4(',', 'v', '1', ':');
        dump_long(val[1]);
        dump_char('\n');

        for (i = 0; i < val.length; i++) {
            for (j = 0; j < val.length; j++) {
                dump_char2('i', ':');
                dump_long(i);
                dump_char3(',', 'j', ':');
                dump_long(j);
                dump_char4(',', 'v', 'i', ':');
                dump_long(val[i]);
                dump_char4(',', 'v', 'j', ':');
                dump_long(val[j]);
                dump_char('\n');
                if ((byte) val[i] == val[i] && (byte) val[j] == val[j]) {
                    test_byte((byte)val[i], (byte)val[j]);
                }
                if ((short) val[i] == val[i] && (short) val[j] == val[j]) {
                    test_short((short)val[i], (short)val[j]);
                }
                if ((int) val[i] == val[i] && (int) val[j] == val[j]) {
                    test_int((int)val[i], (int)val[j]);
                }
                test_long(val[i], val[j]);
            }
        }
    }
}
