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
    /// Vert profond, réservé aux actions et aux indicateurs.
    static let vert = Color(red: 0.055, green: 0.42, blue: 0.31)
    static let vertVif = Color(red: 0.07, green: 0.57, blue: 0.43)
    static let vertClair = Color(red: 0.07, green: 0.57, blue: 0.43).opacity(0.12)
    /// Bleu réservé à ce qui requiert une tierce personne.
    static let aide = Color(red: 0.18, green: 0.33, blue: 0.50)
    static let aideClair = Color(red: 0.18, green: 0.33, blue: 0.50).opacity(0.12)
    /// Teintes de la zone sombre qui porte la séance du jour.
    static let sombreHaut = Color(red: 0.055, green: 0.239, blue: 0.184)
    static let sombreBas = Color(red: 0.027, green: 0.125, blue: 0.098)
    static let surSombre = Color(red: 0.949, green: 0.969, blue: 0.961)
    static let surSombreDouce = Color(red: 0.663, green: 0.769, blue: 0.729)
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
let rayonHemiFit: CGFloat = 18

/// Style commun : bouton principal large, facile à toucher d'une seule main.
struct BoutonLargeStyle: ButtonStyle {
    /// `true` sur fond clair (bouton vert), `false` sur la carte sombre
    /// (bouton clair, pour garder un contraste maximal).
    var degrade = true
    var couleurFond: Color = Color(.secondarySystemBackground)
    var couleurTexte: Color = .white

    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(.title3.weight(.bold))
            .frame(maxWidth: .infinity, minHeight: 64)
            .background {
                RoundedRectangle(cornerRadius: 14)
                    .fill(degrade ? AnyShapeStyle(Color.vert) : AnyShapeStyle(couleurFond))
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
