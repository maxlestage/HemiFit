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
    private var soir: Seance { Catalogue.seanceDuSoir() }

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
                    Text("Chaque petit mouvement compte. On avance en douceur.")
                        .foregroundStyle(.secondary)

                    if let absence = Statistiques.joursDepuisDerniereSeance(journal), absence >= 7 {
                        carteRetour(absence)
                    }

                    let serie = Statistiques.serieEnCours(journal)
                    if serie > 0 {
                        carteSerie(serie)
                    }

                    carteSeance(
                        seance,
                        surtitre: "Séance du jour",
                        symbole: "clock",
                        vedette: true,
                        libelleBouton: Statistiques.seanceFaiteAujourdhui(journal)
                            ? "Refaire la séance"
                            : "Commencer la séance"
                    )

                    if Statistiques.seanceFaiteAujourdhui(journal) {
                        carteDejaFaite
                    }

                    carteSeance(
                        soir,
                        surtitre: "Séance du soir",
                        symbole: "moon",
                        vedette: false,
                        libelleBouton: "Ouvrir la séance du soir"
                    )

                    bandeauSecurite
                }
                .padding()
            }
            .navigationTitle(salutation)
            .fullScreenCover(item: $seanceEnCours) { seance in
                SeanceGuideeView(seance: seance)
            }
        }
    }

    private func carteRetour(_ absence: Int) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Content de vous revoir")
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
            HStack(spacing: 14) {
                Image(systemName: "flame.fill")
                    .font(.title2)
                    .foregroundStyle(Color.vert)
                    .frame(width: 52, height: 52)
                    .background(Color.vertClair, in: .rect(cornerRadius: 16))
                Text("\(serie)")
                    .font(.system(size: 40, weight: .bold, design: .rounded))
                    .foregroundStyle(Color.vert)
            }
            Text(serie == 1
                ? "jour de suite. La régularité commence ici."
                : "jours de suite. La régularité, c'est votre force.")
                .foregroundStyle(.secondary)
        }
        .carteHemiFit()
    }

    private func carteSeance(
        _ laSeance: Seance,
        surtitre: String,
        symbole: String,
        vedette: Bool,
        libelleBouton: String
    ) -> some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack {
                Label(
                    "\(surtitre.uppercased()) · \(laSeance.dureeMinutes) MIN",
                    systemImage: symbole
                )
                .font(.caption.weight(.bold))
                .foregroundStyle(vedette ? Color.surSombreDouce : Color.secondary)

                Spacer()

                PastilleRealisation(realisation: laSeance.realisation)
            }

            Text(laSeance.titre)
                .font(.title.bold())
                .foregroundStyle(vedette ? Color.surSombre : Color.primary)

            Text(laSeance.description)
                .foregroundStyle(vedette ? Color.surSombreDouce : Color.secondary)

            Text(laSeance.realisation == .autonome
                 ? "\(laSeance.exercices.count) exercices, réalisables seul, assis ou allongé."
                 : "\(laSeance.exercices.count) exercices, réalisés par la personne qui vous accompagne.")
                .font(.subheadline)
                .foregroundStyle(vedette ? Color.surSombreDouce : Color.secondary)

            Button {
                seanceEnCours = laSeance
            } label: {
                Label(libelleBouton, systemImage: "play.fill")
            }
            .buttonStyle(BoutonLargeStyle(
                degrade: !vedette,
                couleurFond: .surSombre,
                couleurTexte: .sombreBas
            ))
            .padding(.top, 6)
        }
        .carteHemiFit(vedette: vedette)
    }

    private var carteDejaFaite: some View {
        VStack(alignment: .leading, spacing: 4) {
            Label("Séance du jour déjà faite", systemImage: "checkmark.circle.fill")
                .font(.headline)
                .foregroundStyle(Color.vert)
            Text("L'important est la régularité, pas la quantité. Reposez-vous.")
                .foregroundStyle(.secondary)
        }
        .carteHemiFit()
    }

    private var bandeauSecurite: some View {
        Label {
            Text("HemiFit accompagne votre rééducation mais ne remplace pas votre kinésithérapeute ni votre médecin. Faites-leur valider ces exercices, et arrêtez tout mouvement qui fait mal.")
        } icon: {
            Image(systemName: "info.circle")
        }
            .font(.subheadline)
            .padding()
            .frame(maxWidth: .infinity, alignment: .leading)
            .background(.yellow.opacity(0.15), in: .rect(cornerRadius: rayonHemiFit))
    }
}

extension Seance: Identifiable {
    var id: String { titre }
}

/// Pastille indiquant qui réalise l'exercice ou la séance.
struct PastilleRealisation: View {
    let realisation: Realisation

    var body: some View {
        Label(realisation.court, systemImage: realisation.symbole)
            .font(.caption.weight(.bold))
            .padding(.horizontal, 11)
            .padding(.vertical, 5)
            .background(
                realisation == .autonome ? Color.vertClair : Color.aideClair,
                in: .capsule
            )
            .foregroundStyle(realisation == .autonome ? Color.vert : Color.aide)
    }
}

extension View {
    /// Carte standard : fond, coins arrondis et ombre douce.
    func carteHemiFit(vedette: Bool = false) -> some View {
        self
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(vedette ? 24 : 20)
            .background {
                if vedette {
                    RoundedRectangle(cornerRadius: rayonHemiFit)
                        .fill(.degradeSombre)
                        .shadow(color: .sombreBas.opacity(0.28), radius: 24, y: 10)
                } else {
                    RoundedRectangle(cornerRadius: rayonHemiFit)
                        .fill(.background.secondary)
                        .shadow(color: .black.opacity(0.04), radius: 6, y: 2)
                }
            }
            .overlay {
                if !vedette {
                    RoundedRectangle(cornerRadius: rayonHemiFit)
                        .strokeBorder(.separator.opacity(0.4), lineWidth: 0.5)
                }
            }
    }
}

#Preview {
    AccueilView()
        .modelContainer(for: JournalSeance.self, inMemory: true)
}
