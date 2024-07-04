let result = (open foo.txt | find "ABC" | get 0 | ansi strip)
if ($result == "ABC") {
    exit 0
} else if ($result == "abc") {
    print "Output incorrectly converted to lowercase."
    exit 1
}
else {
    print $"Unknown error, result: ($result)"
    exit 2
}

