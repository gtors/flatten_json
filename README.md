# Usage example

```rust
extern crate flatten_json;

use flatten_json::flatten_from_str;

fn main() {
    let json = r#"
        {
            "user": {
                "name": "tom",
                "id": 115026,
            },
            "role": "AUTHOR",
            "status": "APPROVED"
        }
    "#;

    let flat_json = flatten_from_str(json).unwrap();
    println!("{}", flat_json);
    // {"user.name":"tom","user.id":115026,"role":"AUTHOR","status":"APPROVED"}
}
```

Output:
```
{
    "user.name": "tom",
    "user.id": 115026,
    "role": "AUTHOR",
    "status":"APPROVED"
}
```
