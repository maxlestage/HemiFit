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
    static let vert = Color(red: 0.07, green: 0.47, blue: 0.35)
    /// Vert plus lumineux, pour les dégradés.
    static let vertVif = Color(red: 0.09, green: 0.64, blue: 0.48)
    static let vertClair = Color(red: 0.09, green: 0.64, blue: 0.48).opacity(0.15)
}

extension ShapeStyle where Self == LinearGradient {
    /// Dégradé signature, utilisé sur les boutons et les jauges.
    static var degradeAccent: LinearGradient {
        LinearGradient(
            colors: [.vertVif, .vert],
            startPoint: .topLeading,
            endPoint: .bottomTrailing
        )
    }
}

/// Rayon commun des cartes et des boutons.
let rayonHemiFit: CGFloat = 24

/// Style commun : bouton principal large, facile à toucher d'une seule main.
struct BoutonLargeStyle: ButtonStyle {
    var degrade = true
    var couleurFond: Color = .vertClair
    var couleurTexte: Color = .white

    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(.title3.weight(.bold))
            .frame(maxWidth: .infinity, minHeight: 64)
            .background {
                if degrade {
                    RoundedRectangle(cornerRadius: rayonHemiFit)
                        .fill(.degradeAccent)
                        .shadow(color: .vert.opacity(0.3), radius: 12, y: 6)
                } else {
                    RoundedRectangle(cornerRadius: rayonHemiFit)
                        .fill(couleurFond)
                }
            }
            .foregroundStyle(couleurTexte)
            .scaleEffect(configuration.isPressed ? 0.975 : 1)
            .animation(.spring(duration: 0.25), value: configuration.isPressed)
    }
}

#Preview {
    ContentView()
        .modelContainer(for: JournalSeance.self, inMemory: true)
}
