// pub fn serialize_snbt(nbt: &NbtValue, pretty: bool) -> String {
//     match &nbt.data {
//         NbtValueData::Map(val) => {
//             let mut parts = Vec::new();

//             for (k, v) in val {
//                 if pretty {
//                     parts.push(format!("{k}: {},", serialize_snbt(&v, pretty)).indent(4));
//                 } else {
//                     parts.push(format!("{k}: {},", serialize_snbt(&v, pretty)));
//                 }
//             }

//             if pretty {
//                 format!("{{\n{}\n}}", parts.join("\n"))
//             } else {
//                 format!("{{{}}}", parts.join(" ").trim_end_matches(","))
//             }
//         }

//         NbtValueData::Array(val) => format!(
//             "{}{}{}",
//             if pretty { "[\n" } else { "[" },
//             val.iter()
//                 .map(|it| if pretty {
//                     format!("{},", serialize_snbt(it, pretty)).indent(4)
//                 } else {
//                     format!("{},", serialize_snbt(it, pretty))
//                 })
//                 .collect::<Vec<_>>()
//                 .join(if pretty { "\n" } else { " " })
//                 .trim_end_matches(if pretty { "" } else { "," }),
//             if pretty { "\n]" } else { "]" },
//         ),

//         NbtValueData::String(val) => format!("\"{val}\""),
//         NbtValueData::Float(val) => format!("{val}f"),
//         NbtValueData::Double(val) => format!("{val}d"),
//         NbtValueData::Int(val) => format!("{val}"),
//         NbtValueData::Long(val) => format!("{val}L"),
//         NbtValueData::Bool(val) => format!("{}b", if *val { 1 } else { 0 }),
//         NbtValueData::Byte(val) => format!("{val}b"),
//         NbtValueData::Expr(val) => format!("{val}"),
//     }
// }
