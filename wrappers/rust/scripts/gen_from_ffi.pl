#!/usr/bin/perl

use v5.18;

my $namespace = $ARGV[0];
my $text = $ARGV[1];

if (-f $text)
{
    open(my $FH, $text) or die("Unable to open file $text");
    binmode($FH);
    read($FH, $text, -s $FH);
    close($FH);
}

my $body = "";
my $ffi_namespace = lc($namespace);
my @ffi_callbacks = ();
my %used_ffi_callbacks = ();
while ($text =~ m/(pub fn \w+\([^)]+\)\s*->\s*Error;)/go)
{
    my $method = $1;
    my ($name, $params) = $method =~ m/pub fn (\w+)\(([^)]+)\)/g;
    $method =~ s/\s+/ /go;
    $method =~ s/indy_//go;
    my $method_name = $name;
    $name =~ s/indy_//go;
    $name =~ s/_?\Q$namespace\E//goi;
    $params =~ s/\s+/ /go;
    $params =~ s/CString/&str/go;
    $params =~ s/CVoid//go;
    $params =~ s/\bHandle\b/IndyHandle/go;
    my $simple_params = $params;
    $simple_params =~ s/command_handle:\s*\w+,\s*//go;
    $simple_params =~ s/\s*,\s*cb:\s*[<>\w]+//go;
    
    my $params_no_types = $simple_params;
    $params_no_types =~ s/:\s*[\[\]&<>\w]+(?=,|$)//go;

    my ($ffi_callback, $callback_result) = $params =~ m/Option<(Response(\w+)CB)>/goi;
    if (!exists $used_ffi_callbacks{$ffi_callback})
    {
        $used_ffi_callbacks{$ffi_callback} = 1;
        push(@ffi_callbacks, $ffi_callback);
    }
    my $closure_handler = "";
    my $result_handler = lc($callback_result);
    if ($callback_result eq "Empty")
    {
        $callback_result = "()";
    }
    else
    {
        $closure_handler = $callback_result =~ s/([A-Z])/_$1/gro;   
        $closure_handler = lc($closure_handler);

        $callback_result =~ s/([A-Z])/, $1/go;
        $callback_result = substr($callback_result, 2);
        $callback_result =~ s/Bool/bool/go;
        $callback_result =~ s/I(\d\d)/i$1/go;
        $callback_result =~ s/U(\d\d)/u$1/go;
        my @count = $callback_result =~ m/,/go;
        if (@count == 0)
        {
            $result_handler = "one";
        }
        elsif (@count == 1)
        {
            $result_handler = "two";
            $callback_result = "($callback_result)";
        }
        elsif (@count == 2)
        {
            $result_handler = "three";
            $callback_result = "($callback_result)";
        }
        else
        {
            die("Unknown result handler ". @count);
        }
    }
    
    $body .= "    pub fn $name($simple_params) -> Result<$callback_result, ErrorCode> {\n";
    $body .= "        let (receiver, command_handle, cb) = ClosureHandler::cb_ec$closure_handler();\n";
    $body .= "\n";
    $body .= "        let err = $namespace" ."::_". $name . "(command_handle, $params_no_types, cb);\n";
    $body .= "\n";
    $body .= "        ResultHandler::$result_handler(err, receiver)\n";
    $body .= "    }\n";
    $body .= "\n";
    $body .= "    /// * `timeout` - the maximum time this function waits for a response\n";
    $body .= "    pub fn $name\_timeout($simple_params, timeout: Duration) -> Result<$callback_result, ErrorCode> {\n";
    $body .= "        let (receiver, command_handle, cb) = ClosureHandler::cb_ec$closure_handler();\n";
    $body .= "\n";
    $body .= "        let err = $namespace" ."::_". $name . "(command_handle, $params_no_types, cb);\n";
    $body .= "\n";
    $body .= "        ResultHandler::$result_handler\_timeout(err, receiver, timeout)\n";
    $body .= "    }\n";
    $body .= "\n";
    $body .= "    /// * `closure` - the closure that is called when finished\n";
    $body .= "    ///\n";
    $body .= "    /// # Returns\n";
    $body .= "    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result\n";
    if ($callback_result =~ m/^\(/o)
    {
        $callback_result = substr($callback_result, 1);
        chop($callback_result);
    }
    if ($result_handler eq "empty")
    {
        $body .= "    pub fn $name\_async<F: 'static>($simple_params, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {\n";
    }
    else
    {
        $body .= "    pub fn $name\_async<F: 'static>($simple_params, closure: F) -> ErrorCode where F: FnMut(ErrorCode, $callback_result) + Send {\n";
    }
    $body .= "        let (command_handle, cb) = ClosureHandler::convert_cb_ec$closure_handler(Box::new(closure));\n";
    $body .= "\n";
    $body .= "        $namespace\::_$name(command_handle, $params_no_types, cb)\n";
    $body .= "    }\n\n";
    $body .= "    fn _$name($params) -> ErrorCode {\n";
    while ($params =~ m/(\w+):\s*\&str/go)
    {
        $body .= "        let $1 = c_str!($1);\n";
    }
    while ($params =~ m/(\w+):\s*Option<\&str>/go)
    {
        $body .= "        let $1\_str = opt_c_str!($1);\n";
    }
    $body .= "\n";
    $body .= "        ErrorCode::from(unsafe {\n";
    $body .= "          $ffi_namespace\::$method_name(";
    while ($params =~ m/(\w+):\s*([&\[\]<>\w]+)(,|$)/go)
    {
        my $param_name = $1;
        my $param_type = $2;
        my $delimiter = $3;
    
        if ($param_type eq "&str")
        {
            $body .= "$1.as_ptr()$delimiter";
        }
        elsif ($param_type eq "Option<&str>")
        {
            $body .= "opt_c_ptr!($1,$1\_str)$delimiter";
        }
        elsif ($param_type eq "&[u8]")
        {
            $body .= "$1.as_ptr() as *const u8, $1.len() as u32$delimiter";
        }
        else
        {
            $body .= "$1$delimiter";
        }
        if ($delimiter)
        {
            $body .= " ";
        }
    }
    $body .= ")\n";
    $body .= "        })\n";
    $body .= "    }\n\n";
}

say "\n\n";
say "use ffi::". lc($namespace) . ";";
if (@ffi_callbacks > 1)
{
    say "use ffi::{" . join(", ", @ffi_callbacks) . "};";
}
else
{
    say "use ffi::". $ffi_callbacks[0] . ";";
}
say "\n";
say "pub struct $namespace {}\n";
say "impl $namespace {";
$body =~ s/\s+$//g;
say $body;
say "}";
