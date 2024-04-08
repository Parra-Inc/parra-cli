/// All the templates used in the project generator. Done like this instead of
/// having a bunch of files in a templates directory to simplify packaging a
/// standalone binary.
///

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
    entitlements:
      path: App/App.entitlements
      properties:
        com.apple.developer.aps-environment: development
    settings:
      base:
        GENERATE_INFOPLIST_FILE: YES
        CODE_SIGN_IDENTITY: "-"
        CODE_SIGNING_REQUIRED: NO
        CODE_SIGNING_ALLOWED: NO
        PRODUCT_BUNDLE_IDENTIFIER: {{ app.bundle_id }}
        DEVELOPMENT_ASSET_PATHS: "\"{{ app.name }}/Preview Content\""
        INFOPLIST_KEY_UILaunchScreen_Generation: YES
        INFOPLIST_KEY_UIApplicationSceneManifest_Generation: YES
        INFOPLIST_KEY_UIApplicationSupportsIndirectInputEvents: YES
        INFOPLIST_KEY_UISupportedInterfaceOrientations: "UIInterfaceOrientationLandscapeLeft UIInterfaceOrientationLandscapeRight UIInterfaceOrientationPortrait"
        INFOPLIST_KEY_UISupportedInterfaceOrientations_iPad: "UIInterfaceOrientationLandscapeLeft UIInterfaceOrientationLandscapeRight UIInterfaceOrientationPortrait UIInterfaceOrientationPortraitUpsideDown"
    dependencies:
      - package: Parra

settings:
  CODE_SIGNING_ALLOWED: NO
  base:
    SWIFT_VERSION: 5.9
    MARKETING_VERSION: 1.0.0
    CURRENT_PROJECT_VERSION: 1    
  debug:
    CODE_SIGN_IDENTITY: "-"
    CODE_SIGNING_REQUIRED: NO
    CODE_SIGNING_ALLOWED: NO
  release:
    CODE_SIGN_IDENTITY: iPhone Distribution

packages:
  Parra:
    url: https://github.com/Parra-Inc/parra-ios-sdk
    minorVersion: 0.1.3

"#
    .to_string();
}

pub fn get_app_swift_template() -> String {
    return r#"//
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
                workspaceId: "{{ tenant.id }}",
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
    return r#"//
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

pub fn get_assets_json() -> String {
    return r#"{
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}"#
    .to_string();
}

pub fn get_accent_color_json() -> String {
    return r#"{
  "colors" : [
    {
      "idiom" : "universal"
    }
  ],
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}
"#
    .to_string();
}

pub fn get_app_icon_json() -> String {
    return r#"{
  "images" : [
    {
      "idiom" : "universal",
      "platform" : "ios",
      "size" : "1024x1024"
    }
  ],
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}
"#
    .to_string();
}
