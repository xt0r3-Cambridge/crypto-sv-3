#!/usr/bin/perl
use bigint; # use arbitrary-length integer type
sub modinv {
my ($a0, $b0) = @_;
my ( $a, $b) = @_;
my ($aa, $ab, $ba, $bb) = (1, 0, 0, 1);
$a0 = $a0 % $b0;
while (1) {
    my $q = $a / $b;
    # print "$a, $b, $q\n";
    if ($a == $b * $q) {
        if ($b == 1){
            print "inv($a0) = $ba\n";
            return $ba;
        }
        else {
            die "No modular inverse found, gcd($a0,$b0) = $b\n";
        }
    }
    ($a, $b, $aa, $ab, $ba, $bb) =
    ($b, ($a-$b*$q) , $ba % $b0, $bb % $b0, ($aa-$ba*$q) % $b0, ($ab-$bb*$q) % $b0);
    }
}
# modinv(806515533049393, 1304969544928657);
# modinv(4505490,7290036);
modinv(892302390667940581330701, 1208925819614629174706111);