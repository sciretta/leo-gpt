//
//  HomeView.swift
//  LeoGPT
//
//  Created by Leonardo on 23/11/25.
//

import SwiftUI

struct HomeView: View {
    @State private var items : [Chat] = []
    @State private var goToChat = false
    @EnvironmentObject var appState: AppState
    @State private var chatId: String = ""
    

    var body: some View {
        VStack(alignment: .center) {
            // Hidden navigation link to programmatically navigate
            NavigationLink(
                destination: ChatView(chatId:chatId).environmentObject(appState).navigationTitle("Conversation"),
                isActive: $goToChat
            ) { EmptyView() }

            Button(action: {  chatId =
                "";goToChat = true; }) {
                Text("Start a conversation")
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(width: 300, height: 60)
                    .background(Colors.primary)
                    .cornerRadius(12)
                    
            }
            .padding(.bottom, 16)

            Text("Last conversations")
                .font(.title2)
                .bold()

            ScrollView {
                VStack(alignment: .leading, spacing: 12) {
                    ForEach(items, id: \.self) { item in
                        Button {
                            chatId = item.id.oid;
                            goToChat = true
                        } label: {
                            Text(item.chatName)
                                .padding()
                                .frame(maxWidth: .infinity, alignment: .center)
                                .background(Colors.secondary.opacity(0.1))
                                .cornerRadius(10)
                                .overlay(
                                    RoundedRectangle(cornerRadius: 10)
                                        .stroke(Colors.primary, lineWidth: 2)
                                )
                        }
                    }
                }
                .padding(.vertical, 4)
            }
            .frame(width: 300, height: 400) // Fixed size here
            .background(Colors.terciary.opacity(0.1))
            .cornerRadius(12)
        }
        .padding()
        .onAppear {
            Task {
                do {
                    let chats = try await fetchChats(username: appState.username)
                    items = chats
                    print("UserId: \(chats[0].userId.oid)")
                    appState.userId = chats[0].userId.oid
                } catch {
                    print("Error", error)
                }
            }
        }
    }
}

#Preview {
    HomeView().environmentObject(AppState())
}
