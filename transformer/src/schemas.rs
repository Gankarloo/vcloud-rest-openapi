use anyhow::{Context, Result};
use indexmap::IndexMap;
use openapiv3::{ReferenceOr, Schema};
use std::collections::BTreeMap;
use std::io::{Read, Seek};
use xmltree::XMLNode;

use zip::read::ZipArchive;

pub fn schemas<R: Read + Seek>(
    output: &mut IndexMap<String, ReferenceOr<Schema>>,
    zip: &mut ZipArchive<R>,
) -> Result<BTreeMap<String, String>> {
    let mut content_type_mapping = BTreeMap::new();
    
    // Collect schema file names first to enable sorting and avoid borrow conflicts
    let mut type_file_names: Vec<String> = zip
        .file_names()
        .filter(|name| {
            name.starts_with("doc/etc/")
                && name.ends_with(".xsd")
                && !name.starts_with("doc/etc/etc/snapshot")
                && *name != "doc/etc/schemas/external/xml.xsd"
                && *name != "doc/etc/etc/schemas/external/xml.xsd"
        })
        .map(|name| name.to_string())
        .collect();

    type_file_names.sort();

    // Process each schema file
    let mut all_types = Vec::new();
    for file_name in &type_file_names {
        let mut buffer = Vec::new();
        let mut file = zip.by_name(file_name)
            .with_context(|| format!("Failed to access file: {}", file_name))?;
        file.read_to_end(&mut buffer)
            .with_context(|| format!("Failed to read file: {}", file_name))?;

        if let Ok(xml) = xmltree::Element::parse(&buffer as &[u8]) {
            let ns = xml.attributes.get("targetNamespace").map(|t| match t.as_str() {
                "http://schemas.dmtf.org/ovf/envelope/1" => "ovf",
                "http://schemas.dmtf.org/ovf/environment/1" => "ovfenv",
                "http://schemas.dmtf.org/wbem/wscim/1/cim-schema/2/CIM_ResourceAllocationSettingData" => "rasd",
                "http://schemas.dmtf.org/wbem/wscim/1/cim-schema/2/CIM_VirtualSystemSettingData" => "vssd",
                "http://schemas.dmtf.org/wbem/wscim/1/common" => "cim",
                "http://www.vmware.com/vcloud/meta" => "meta",
                "http://www.vmware.com/schema/ovf" => "vmw",
                "http://www.vmware.com/vcloud/extension/v1.5" => "vcloud-ext",
                "http://www.vmware.com/vcloud/v1.5" => "vcloud",
                "http://www.vmware.com/vcloud/versions" => "versioning",
                _ => "vcloud",
            });
            
            all_types.push((ns, XMLNode::Element(xml)));
        }
    }

    // Sort by namespace for consistent output
    all_types.sort_by_key(|(ns, _)| ns.unwrap_or(""));

    // Process all schemas
    for (ns, type_xml) in all_types {
        let xsd_schema = crate::parsers::doc::etc::schema::Schema::try_from((ns, &type_xml, &Vec::new()))
            .with_context(|| format!("Failed to parse schema for namespace: {:?}", ns))?;
        
        output.extend(
            Vec::<Schema>::from(&xsd_schema)
                .into_iter()
                .filter_map(|s| {
                    s.schema_data
                        .title
                        .clone()
                        .map(|title| (title, ReferenceOr::Item(s)))
                }),
        );
        
        content_type_mapping.extend(xsd_schema.content_types_names());
    }

    Ok(content_type_mapping)
}
