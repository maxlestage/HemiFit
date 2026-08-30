//
//  AccueilView.swift
//  HemiFit
//

import SwiftData
import SwiftUI

struct AccueilView: View {
    @Query(sort: \JournalSeance.date, order: .reverse) private var journal: [JournalSeance]
    @State private var seanceEnCours: Seance?

    private var seance: Seance { Catalogue.seanceDuJour() }

    private var salutation: String {
        let heure = Calendar.current.component(.hour, from: .now)
        if heure < 12 { return "Bonjour" }
        if heure < 18 { return "Bon après-midi" }
        return "Bonsoir"
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 16) {
                    Text("Chaque petit mouvement compte. On y va en douceur.")
                        .foregroundStyle(.secondary)

                    if let absence = Statistiques.joursDepuisDerniereSeance(journal), absence >= 7 {
                        carteRetour(absence)
                    }

                    let serie = Statistiques.serieEnCours(journal)
                    if serie > 0 {
                        carteSerie(serie)
                    }

                    carteSeanceDuJour

                    if Statistiques.seanceFaiteAujourdhui(journal) {
                        carteDejaFaite
                    }

                    bandeauSecurite
                }
                .padding()
            }
            .navigationTitle("\(salutation) 👋")
            .fullScreenCover(item: $seanceEnCours) { seance in
                SeanceGuideeView(seance: seance)
            }
        }
    }

    private func carteRetour(_ absence: Int) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Content de vous revoir 💚")
                .font(.title3.bold())
            Text("\(absence >= 60 ? "Cela fait un moment, et ce n'est pas grave du tout." : "Quelques jours sans séance, et alors ?") Une pause n'efface rien de ce que vous avez déjà construit — votre meilleure série reste inscrite dans vos progrès.")
                .foregroundStyle(.secondary)
            Text("On reprend tranquillement, là où vous en êtes aujourd'hui. C'est le seul endroit d'où on peut repartir.")
                .foregroundStyle(.secondary)
        }
        .carteHemiFit()
    }

    private func carteSerie(_ serie: Int) -> some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("🔥 \(serie)")
                .font(.system(size: 40, weight: .bold))
                .foregroundStyle(Color.vert)
            Text(serie == 1
                ? "jour de suite — bien joué, la régularité commence ici !"
                : "jours de suite — la régularité, c'est votre superpouvoir.")
                .foregroundStyle(.secondary)
        }
        .carteHemiFit()
    }

    private var carteSeanceDuJour: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Séance du jour · \(seance.dureeMinutes) min environ")
                .font(.subheadline.weight(.semibold))
                .padding(.horizontal, 14)
                .padding(.vertical, 6)
                .background(Color.vertClair, in: .capsule)
                .foregroundStyle(Color.vert)

            Text(seance.titre)
                .font(.title2.bold())

            Text(seance.description)
                .foregroundStyle(.secondary)

            Text("\(seance.exercices.count) exercices, tous assis ou allongé, avec l'aide de votre main gauche.")
                .foregroundStyle(.secondary)

            Button {
                seanceEnCours = seance
            } label: {
                Label(
                    Statistiques.seanceFaiteAujourdhui(journal)
                        ? "Refaire une séance"
                        : "Commencer la séance",
                    systemImage: "play.fill"
                )
            }
            .buttonStyle(BoutonLargeStyle())
            .padding(.top, 4)
        }
        .carteHemiFit(vedette: true)
    }

    private var carteDejaFaite: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("✅ Séance du jour déjà faite")
                .font(.headline)
            Text("Bravo ! Reposez-vous, l'important c'est la régularité, pas la quantité.")
                .foregroundStyle(.secondary)
        }
        .carteHemiFit()
    }

    private var bandeauSecurite: some View {
        Text("⚕️ HemiFit accompagne votre rééducation mais ne remplace pas votre kinésithérapeute ni votre médecin. Montrez-leur ces exercices, et arrêtez tout mouvement qui fait mal.")
            .font(.subheadline)
            .padding()
            .frame(maxWidth: .infinity, alignment: .leading)
            .background(.yellow.opacity(0.15), in: .rect(cornerRadius: rayonHemiFit))
    }
}

extension Seance: Identifiable {
    var id: String { titre }
}

extension View {
    /// Carte standard : fond, coins arrondis et ombre douce.
    func carteHemiFit(vedette: Bool = false) -> some View {
        self
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(22)
            .background {
                RoundedRectangle(cornerRadius: rayonHemiFit)
                    .fill(.background.secondary)
                    .shadow(
                        color: .black.opacity(vedette ? 0.10 : 0.05),
                        radius: vedette ? 20 : 10,
                        y: vedette ? 8 : 4
                    )
            }
            .overlay {
                RoundedRectangle(cornerRadius: rayonHemiFit)
                    .strokeBorder(.separator.opacity(0.5), lineWidth: 0.5)
            }
    }
}

#Preview {
    AccueilView()
        .modelContainer(for: JournalSeance.self, inMemory: true)
}
