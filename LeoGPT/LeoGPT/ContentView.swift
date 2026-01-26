//
//  ContentView.swift
//  LeoGPT
//
//  Created by Leonardo on 16/11/25.
//

import SwiftUI

class AppState: ObservableObject {
    @Published var nickname: String = ""
}

struct ContentView: View {
    var body: some View {
        NavigationStack {
            HomeView()
        }
    }
}

#Preview {
    ContentView()
}
