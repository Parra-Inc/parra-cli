/// All the templates used in the project generator. Done like this instead of
/// having a bunch of files in a templates directory to simplify packaging a
/// standalone binary.

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
    dependencies:
      - package: Parra

settings:
  GENERATE_INFOPLIST_FILE: NO
  CODE_SIGNING_ALLOWED: NO
  MARKETING_VERSION: 1.0.0
  base:
    CURRENT_PROJECT_VERSION: 1

packages:
  Parra:
    url: https://github.com/Parra-Inc/parra-ios-sdk
    minorVersion: 0.1.3

"#
    .to_string();
}

pub fn get_app_swift_template() -> String {
    return r#"
//
//  {{ app.camel_name }}App.swift
//  {{ app.name }}
//
//  Bootstrapped with ❤️ by Parra on {{ "now" | date: "%m/%d/%Y" }}.
//  Copyright © {{ "now" | date: "%Y" }} {{ tenant.name }}. All rights reserved.
//

import Parra
import SwiftUI

@main
final class {{ app.camel_name }}App: ParraApp<ParraAppDelegate, ParraSceneDelegate> {
    required init() {
        super.init()

        configureParra(
            authProvider: .default(
                tenantId: "{{ tenant.id }}",
                applicationId: "{{ app.id }}",
                authProvider: {
                    fatalError("You must implement your own authentication provider")
                }
            ),
            appContent: {
                ContentView()
            }
        )
    }
}

"#
    .to_string();
}

pub fn get_content_view_swift_template() -> String {
    return r#"
//
//  ContentView.swift
//  {{ app.name }}
//
//  Bootstrapped with ❤️ by Parra on {{ "now" | date: "%m/%d/%Y" }}.
//  Copyright © {{ "now" | date: "%Y" }} {{ tenant.name }}. All rights reserved.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
        }
        .padding()
    }
}

#Preview {
    ContentView()
}

"#
    .to_string();
}
