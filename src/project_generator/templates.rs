/// All the templates used in the project generator. Done like this instead of
/// having a bunch of files in a templates directory to simplify packaging a
/// standalone binary.
///
/// https://github.com/yonaskolb/XcodeGen/tree/master
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
    iOS: {{ app.deployment_target }}
targets:
  {{ app.name }}:
    type: application
    platform: iOS
    deploymentTarget: {{ app.deployment_target }}
    sources: [{{ app.name }}]
    info:
      path: {{ app.name }}/Info.plist
      properties:
        UISupportedInterfaceOrientations: [UIInterfaceOrientationLandscapeLeft, UIInterfaceOrientationLandscapeRight, UIInterfaceOrientationPortrait]
        UISupportedInterfaceOrientations~ipad: [UIInterfaceOrientationLandscapeLeft, UIInterfaceOrientationLandscapeRight, UIInterfaceOrientationPortrait, UIInterfaceOrientationPortraitUpsideDown]
        UILaunchScreen:
          UIImageRespectsSafeAreaInsets: NO
        UIApplicationSceneManifest_Generation: YES
        UIApplicationSupportsIndirectInputEvents: YES
        ITSAppUsesNonExemptEncryption: NO
        NSCameraUsageDescription: "{{ app.name }} requires access to the camera to take photos."
    settings:
      base:
        CODE_SIGN_IDENTITY: "-"
        CODE_SIGNING_REQUIRED: NO
        CODE_SIGNING_ALLOWED: NO
        PRODUCT_BUNDLE_IDENTIFIER: {{ app.bundle_id }}
        DEVELOPMENT_ASSET_PATHS: "\"{{ app.name }}/Preview Content\""
        SWIFT_VERSION: 5.9
        MARKETING_VERSION: 1.0.0
        CURRENT_PROJECT_VERSION: 1
        SWIFT_ENABLE_BARE_SLASH_REGEX: YES
      configs:
        debug:
          CODE_SIGN_ENTITLEMENTS: {{ app.name }}/Entitlements-debug.entitlements
        release:
          CODE_SIGN_ENTITLEMENTS: {{ app.name }}/Entitlements-release.entitlements
    dependencies:
      - package: Parra

settings:
  CODE_SIGNING_ALLOWED: NO
  base:
    SWIFT_VERSION: 5.9
    MARKETING_VERSION: 1.0.0
    CURRENT_PROJECT_VERSION: 1
    SWIFT_ENABLE_BARE_SLASH_REGEX: YES
  debug:
    CODE_SIGN_IDENTITY: "-"
    CODE_SIGNING_REQUIRED: NO
    CODE_SIGNING_ALLOWED: NO
  release:
    CODE_SIGN_IDENTITY: iPhone Distribution

packages:
  Parra:
    url: https://github.com/Parra-Inc/parra-ios-sdk
    minorVersion: 0.1.9

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
            workspaceId: "{{ tenant.id }}",
            applicationId: "{{ app.id }}",
            appContent: {
                ParraRequiredAuthView(
                    authenticatedContent: { _ in
                        ContentView()
                    },
                    unauthenticatedContent: { _ in
                        ParraDefaultAuthenticationFlowView(
                            flowConfig: .default
                        )
                    }
                )
            }
        )
    }
}
"#
    .to_string();
}

pub fn get_entitlements_xml() -> String {
    return r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>aps-environment</key>
	<string>{{ entitlements.aps_environment }}</string>
	<key>com.apple.developer.associated-domains</key>
	<array>
    {{ entitlements.associated_domains }}
	</array>
</dict>
</plist>
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
