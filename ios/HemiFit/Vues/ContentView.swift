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
        .tint(.ardoise)
    }
}

extension Color {
    /// Bleu ardoise : couleur des actions et des indicateurs.
    static let ardoise = Color(red: 0.184, green: 0.333, blue: 0.502)
    static let ardoiseVive = Color(red: 0.227, green: 0.416, blue: 0.624)
    static let ardoiseClaire = Color(red: 0.918, green: 0.941, blue: 0.969)
    /// Vert profond : tout ce qui requiert une tierce personne.
    static let aide = Color(red: 0.173, green: 0.420, blue: 0.345)
    static let aideClair = Color(red: 0.910, green: 0.945, blue: 0.933)
    /// Teintes de la zone sombre qui porte la séance du jour.
    static let sombreHaut = Color(red: 0.118, green: 0.180, blue: 0.243)
    static let sombreBas = Color(red: 0.059, green: 0.098, blue: 0.133)
    static let surSombre = Color(red: 0.957, green: 0.969, blue: 0.980)
    static let surSombreDouce = Color(red: 0.663, green: 0.741, blue: 0.816)
}

extension ShapeStyle where Self == LinearGradient {
    /// Dégradé de la carte sombre : une seule zone sombre par écran.
    static var degradeSombre: LinearGradient {
        LinearGradient(
            colors: [.sombreHaut, .sombreBas],
            startPoint: .topLeading,
            endPoint: .bottomTrailing
        )
    }
}

/// Rayon commun des cartes et des boutons.
let rayonHemiFit: CGFloat = 10

/// Style commun : bouton principal large, facile à toucher d'une seule main.
struct BoutonLargeStyle: ButtonStyle {
    /// `true` sur fond clair (bouton ardoise), `false` sur la carte sombre
    /// (bouton clair, pour garder un contraste maximal).
    var degrade = true
    var couleurFond: Color = Color(.secondarySystemBackground)
    var couleurTexte: Color = .white

    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(.title3.weight(.bold))
            .frame(maxWidth: .infinity, minHeight: 64)
            .background {
                RoundedRectangle(cornerRadius: 8)
                    .fill(degrade ? AnyShapeStyle(Color.ardoise) : AnyShapeStyle(couleurFond))
            }
            .foregroundStyle(degrade ? Color.white : couleurTexte)
            .scaleEffect(configuration.isPressed ? 0.98 : 1)
            .animation(.spring(duration: 0.22), value: configuration.isPressed)
    }
}

#Preview {
    ContentView()
        .modelContainer(for: JournalSeance.self, inMemory: true)
}
