//
//  HomeView.swift
//  LeoGPT
//
//  Created by Leonardo on 23/11/25.
//

import SwiftUI

struct HomeView: View {
    let items = (1...30).map { "Item \($0)" }
    @State private var goToChat = false
    
    var body: some View {
        
        NavigationLink(
            destination: ChatView(),
            isActive: $goToChat
        ) { EmptyView() }
        
        Button(action: {goToChat = true }) {
            Text("Start a conversation")
                .font(.headline)
            
                .foregroundColor(.white)
                .frame(width: 300, height: 60)
                .background(Colors.primary)
                .cornerRadius(12)
        }
        
        VStack(alignment: .center) {
            Text("Last conversations")
                .font(.title2)
                .bold()
            
            ScrollView {
                VStack(alignment: .leading, spacing: 12) {
                    ForEach(items, id: \.self) { item in
                        Button {
                            // Aquí puedes agregar la acción que debe ocurrir al pulsar
                        } label: {
                            Text(item)
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
            .frame(width:300,height: 400)   // 👈 Fixed size here
            .background(Colors.terciary.opacity(0.1))
            .cornerRadius(12)
        }
        .padding()
    }
}

#Preview {
    HomeView()
}
