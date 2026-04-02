use std::fs;
use std::io::Write;

use super::apply::{apply_fomod_selections, default_selections_for};
use super::parse::parse_module_config_str;
use super::types::{FomodGroupSelection, FomodSelections, FomodStepSelection};

const SAMPLE_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<config>
  <moduleName>SampleFomod</moduleName>
  <installSteps order="Explicit">
    <installStep name="Main">
      <optionalFileGroups order="Explicit">
        <group name="Choose" type="SelectExactlyOne">
          <plugins order="Explicit">
            <plugin name="OptionA">
              <description>Pick A</description>
              <files>
                <folder source="payload/a" destination="" priority="0"/>
              </files>
            </plugin>
            <plugin name="OptionB">
              <files>
                <folder source="payload/b" destination="" priority="0"/>
              </files>
            </plugin>
          </plugins>
        </group>
      </optionalFileGroups>
    </installStep>
  </installSteps>
</config>"#;

#[test]
fn parse_sample_module_config() {
    let p = parse_module_config_str(SAMPLE_XML).expect("parse");
    assert_eq!(p.module_name, "SampleFomod");
    assert_eq!(p.steps.len(), 1);
    assert_eq!(p.steps[0].groups[0].plugins.len(), 2);
}

#[test]
fn apply_copies_selected_folder() {
    let tmp = std::env::temp_dir().join(format!("pantheon-fomod-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(tmp.join("fomod")).unwrap();
    fs::write(tmp.join("fomod/ModuleConfig.xml"), SAMPLE_XML).unwrap();
    fs::create_dir_all(tmp.join("payload/a")).unwrap();
    fs::create_dir_all(tmp.join("payload/b")).unwrap();
    let mut f = fs::File::create(tmp.join("payload/a/hello.txt")).unwrap();
    f.write_all(b"aaa").unwrap();

    let parsed = parse_module_config_str(SAMPLE_XML).unwrap();
    let sel = FomodSelections {
        steps: vec![FomodStepSelection {
            step_index: 0,
            groups: vec![FomodGroupSelection {
                group_index: 0,
                plugin_indices: vec![0],
            }],
        }],
    };
    apply_fomod_selections(&tmp, &parsed, &sel).expect("apply");
    assert!(tmp.join("hello.txt").exists());
    assert!(!tmp.join("fomod").exists());
}

#[test]
fn default_selections_selects_first_plugin() {
    let parsed = parse_module_config_str(SAMPLE_XML).unwrap();
    let sel = default_selections_for(&parsed);
    assert_eq!(sel.steps[0].groups[0].plugin_indices, vec![0]);
}
