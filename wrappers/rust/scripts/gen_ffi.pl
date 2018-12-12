#!/usr/bin/perl

use v5.18;

my $text = $ARGV[0];

if (-f $text)
{
    open(my $FH, $text) or die("Unable to open file $text");
    binmode($FH);
    read($FH, $text, -s $FH);
    close($FH);
}

my @usings = qw(Error);

$text =~ s{//.*$}{}mg;
$text =~ s{/\*.*?\*/}{}sg;
$text =~ s/(?:\r?\n)+/\n/g;
if ($text =~ s/\*const c_char/CString/g)
{
    push(@usings, "CString");
}
if ($text =~ s/\*const u8/BString/g)
{
    push(@usings, "BString");
}
if ($text =~ s/i32/Handle/g)
{
    push(@usings, "Handle");
}
if ($text =~ s/\*const c_void/CVoid/g)
{
    push(@usings, "CVoid");
}

$text =~ s/(?<=Error)Code(?:\b|$)//g;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*,\s*\w+:\s*Error\))(?=[>])/ResponseEmptyCB/sg;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*,\s*\w+:\s*Error\s*,\s*\w+:\s*bool\))(?=[>])/ResponseBoolCB/sg;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*,\s*\w+:\s*Error\s*,\s*\w+:\s*Handle\))(?=[>])/ResponseI32CB/sg;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*.\s*\w+:\s*Error\s*,\s*\w+:\s*Handle\s*,\s*\w+:\s*usize\))(?=[>])/ResponseI32UsizeCB/sg;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*,\s*\w+:\s*Error\s*,\s*\w+:\s*CString\))(?=[>])/ResponseStringCB/sg;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*,\s*\w+:\s*Error\s*,\s*\w+:\s*CString,\s*\w+:\s*CString\))(?=[>])/ResponseStringStringCB/sg;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*,\s*\w+:\s*Error\s*,\s*\w+:\s*CString,\s*\w+:\s*CString,\s*\w+:\s*CString\))(?=[>])/ResponseStringStringStringCB/sg;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*,\s*\w+:\s*Error\s*,\s*\w+:\s*BString,\s*\w+:\s*u32\))(?=[>])/ResponseSliceCB/sg;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*,\s*\w+:\s*Error\s*,\s*\w+:\s*CString,\s*\w+:\s*BString,\s*\w+:\s*u32\))(?=[>])/ResponseStringSliceCB/sg;
$text =~ s/(?<=Option<)(extern fn\(\s*\w+:\s*Handle\s*,\s*\w+:\s*Error\s*,\s*\w+:\s*CString,\s*\w+:\s*CString,\s*\w+:\s*u64\))(?=[>])/ResponseStringStringU64CB/sg;

say "use super::*;\n";
say "use {" . join(", ", sort @usings) . "};\n";
say "extern {";
while ($text =~ m/\bpub\s+extern\s+fn\s+(indy_\w+\s*\(.+?\)\s+->\s+Error)/sgo)
{
    my $fn = $1;

    my ($len) = $fn =~ m/^([^(]+)/go;
    my $whitespace = ' ' x (length($len) + length("    pub fn  "));
    $fn =~ s/^\s+(?=\w)/$whitespace/mgo;
    say "";
    say "    #[no_mangle]";
    say "    pub fn $fn;";
}
say "}\n";

while ($text =~ m/\b(pub\s+type\s+\w+\s+=\s+extern\s+fn.+?Error;)/sgo)
{
    say $1;
}
