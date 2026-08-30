//
//  ProgresView.swift
//  HemiFit
//

import SwiftData
import SwiftUI

struct ProgresView: View {
    @Query(sort: \JournalSeance.date, order: .reverse) private var journal: [JournalSeance]

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 16) {
                    Text("La régularité compte plus que la performance.")
                        .foregroundStyle(.secondary)

                    grilleStatistiques

                    carteRienNeSePerd

                    dernieresSeances
                }
                .padding()
            }
            .navigationTitle("Mes progrès")
        }
    }

    private var grilleStatistiques: some View {
        LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 12) {
            tuile(valeur: "\(Statistiques.serieEnCours(journal))", legende: "jours de suite")
            tuile(valeur: "⭐ \(Statistiques.meilleureSerie(journal))", legende: "meilleure série (jamais perdue)")
            tuile(valeur: "\(journal.count)", legende: "séances au total")
            tuile(valeur: "\(Statistiques.minutesTotales(journal))", legende: "minutes de rééducation")
        }
    }

    private func tuile(valeur: String, legende: String) -> some View {
        VStack(spacing: 4) {
            Text(valeur)
                .font(.system(size: 36, weight: .bold, design: .rounded))
                .foregroundStyle(Color.vert)
            Text(legende)
                .font(.subheadline)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity, minHeight: 100)
        .background(.background.secondary, in: .rect(cornerRadius: 20))
    }

    private var carteRienNeSePerd: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text("🌱 Rien de tout cela ne se perd")
                .font(.headline)
            Text("Ces minutes sont du travail réel fait par votre cerveau. Une pause, même de plusieurs mois, ne les efface pas : vous reprendrez là où vous en êtes, jamais à zéro.")
                .foregroundStyle(.secondary)
            if Statistiques.seancesSur7Jours(journal) > 0 {
                let n = Statistiques.seancesSur7Jours(journal)
                Text("Sur les 7 derniers jours : \(n) \(n == 1 ? "séance" : "séances").")
                    .foregroundStyle(.secondary)
            }
        }
        .carteHemiFit()
    }

    private var dernieresSeances: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Dernières séances")
                .font(.title3.bold())

            if journal.isEmpty {
                Text("Aucune séance pour l'instant. La première est la plus importante — elle vous attend sur l'accueil. 💚")
                    .foregroundStyle(.secondary)
            }

            ForEach(journal.prefix(14)) { seance in
                HStack(alignment: .firstTextBaseline) {
                    VStack(alignment: .leading, spacing: 2) {
                        Text(seance.titre)
                            .font(.headline)
                        Text(seance.date.formatted(.dateTime.weekday(.wide).day().month(.wide)))
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                    }
                    Spacer()
                    Text("\(seance.minutes) min \(seance.ressenti?.emoji ?? "")")
                }
                .padding(.vertical, 6)
                Divider()
            }
        }
        .carteHemiFit()
    }
}

#Preview {
    ProgresView()
        .modelContainer(for: JournalSeance.self, inMemory: true)
}
