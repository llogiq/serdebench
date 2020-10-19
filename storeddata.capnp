@0xc36517d21e1c8caf;

# The capnpc tool enforces that fields be camelCase and types PascalCase.

struct StoredData {
    variant :union {
        yesNo @0 :Bool;
        small @1 :UInt8;
        signy @2 :Int64;
        stringy @3 :Text;
    }

    # One way to make a primitive field optional is to wrap it in a union like this.
    # Alternatively, we could move the field to a substruct, but that would take
    # more space and require an extra pointer indirection.
    optBool :union {
      none @4 :Void;
      value @5 :Bool;
    }

    vecStrs @6 :List(Text);
    range @7 :Range;
}

struct Range {
    start @0 :UInt64;
    end @1 :UInt64;
}

