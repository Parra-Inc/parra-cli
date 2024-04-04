pub fn get_project_yaml_template() -> String {
    return r#"
name: {{ app.name }}
options:
  xcodeVersion: 15.3
  minimumXcodeGenVersion: 2.39.0
  createIntermediateGroups: true
  generateEmptyDirectories: true
  deploymentTarget:
    iOS: "17.0"
targets:
  {{ app.name }}:
    type: application
    platform: iOS
    deploymentTarget: "17.0"
    sources: [{{ app.name }}]
    info:
      path: {{ app.name }}/Info.plist
    settings:
      base:
        PRODUCT_BUNDLE_IDENTIFIER: {{ app.bundle_id }}

settings:
  GENERATE_INFOPLIST_FILE: NO
  CODE_SIGNING_ALLOWED: NO
  MARKETING_VERSION: 1.0.0
  base:
    CURRENT_PROJECT_VERSION: 1
"#
    .to_string();
}
