//
//  ContentView.swift
//  HemiFit
//
//  Navigation principale : 4 onglets en bas d'écran,
//  accessibles au pouce gauche.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        TabView {
            Tab("Accueil", systemImage: "house.fill") {
                AccueilView()
            }
            Tab("Exercices", systemImage: "list.bullet.rectangle.fill") {
                ExercicesListeView()
            }
            Tab("Progrès", systemImage: "chart.line.uptrend.xyaxis") {
                ProgresView()
            }
            Tab("Conseils", systemImage: "lightbulb.fill") {
                ConseilsView()
            }
        }
        .tint(.vert)
    }
}

extension Color {
    /// Vert apaisant, couleur signature de HemiFit.
    static let vert = Color(red: 0.12, green: 0.48, blue: 0.36)
    static let vertClair = Color(red: 0.12, green: 0.48, blue: 0.36).opacity(0.14)
}

/// Style commun : bouton principal large, facile à toucher d'une seule main.
struct BoutonLargeStyle: ButtonStyle {
    var couleurFond: Color = .vert
    var couleurTexte: Color = .white

    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(.title3.weight(.semibold))
            .frame(maxWidth: .infinity, minHeight: 64)
            .background(couleurFond, in: .rect(cornerRadius: 20))
            .foregroundStyle(couleurTexte)
            .scaleEffect(configuration.isPressed ? 0.985 : 1)
    }
}

#Preview {
    ContentView()
        .modelContainer(for: JournalSeance.self, inMemory: true)
}
