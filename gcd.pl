#!/usr/bin/perl
use bigint; # use arbitrary-length integer type
sub gcd {
my ($a0, $b0) = @_;
my ( $a, $b) = @_;
my ($aa, $ab, $ba, $bb) = (1, 0, 0, 1);
while (1) {
    my $q = $a / $b;
    if ($a == $b * $q) {
        print "gcd($a0,$b0) = $b = $a0 * $ba + $b0 * $bb\n";
        return $b;
    }
    ($a, $b, $aa, $ab, $ba, $bb) =
    ($b, $a-$b*$q, $ba, $bb, $aa-$ba*$q, $ab-$bb*$q);
    }
}
# gcd(2250,360);
gcd(733810016255931844845,1187329547587210582322);