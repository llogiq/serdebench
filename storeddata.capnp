@0xc36517d21e1c8caf;

# The capnpc tool enforces that fields be camelCase and types PascalCase.

struct StoredData {
    variant :union {
        yesNo @0 :Bool;
        small @1 :UInt8;
        signy @2 :Int64;
        stringy @3 :Text;
    }
    optBool @4 :OptionBool; # We don't store bool directly, because primitive types are given a default (0) value when not present
    vecStrs @5 :List(Text);
    range @6 :Range;
}

struct Range {
    start @0 :UInt64;
    end @1 :UInt64;
}

# All struct types are optional when used as members of another struct
struct OptionBool {
    value @0 :Bool;
}