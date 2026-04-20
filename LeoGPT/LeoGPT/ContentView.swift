//
//  ContentView.swift
//  LeoGPT
//
//  Created by Leonardo on 16/11/25.
//

import SwiftUI

struct MongoID: Codable, Hashable {
    let oid: String

    enum CodingKeys: String, CodingKey {
        case oid = "$oid"
    }
}

struct Chat: Codable, Identifiable, Hashable {
    let id: MongoID
    let userId: MongoID
    let chatName: String
    let createdAt: String

    enum CodingKeys: String, CodingKey {
        case id = "_id"
        case userId = "user_id"
        case chatName = "chat_name"
        case createdAt = "created_at"
    }
}

struct Message: Codable, Identifiable, Hashable {
    let id: MongoID
    let chatId: MongoID
    let isUser: Bool
    let content: String

    enum CodingKeys: String, CodingKey {
        case id = "_id"
        case chatId = "chat_id"
        case isUser = "is_user"
        case content = "content"
    }
}

class AppState: ObservableObject {
    @Published var username: String = "johndoe"
    @Published var userId:String = ""
}

struct ContentView: View {
    var body: some View {
        NavigationStack {
            HomeView()
                .environmentObject(AppState())
        }
    }
}

#Preview {
    ContentView()
}
