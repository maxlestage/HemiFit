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

                    carteSemaine

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
            tuile(symbole: "flame.fill", valeur: "\(Statistiques.serieEnCours(journal))", legende: "jours de suite")
            tuile(symbole: "star.fill", valeur: "\(Statistiques.meilleureSerie(journal))", legende: "meilleure série, jamais perdue")
            tuile(symbole: "checkmark.circle", valeur: "\(journal.count)", legende: "séances au total")
            tuile(symbole: "clock", valeur: "\(Statistiques.minutesTotales(journal))", legende: "minutes de rééducation")
        }
    }

    private func tuile(symbole: String, valeur: String, legende: String) -> some View {
        VStack(spacing: 6) {
            Image(systemName: symbole)
                .font(.subheadline.weight(.semibold))
                .foregroundStyle(Color.vert)
                .frame(width: 36, height: 36)
                .background(Color.vertClair, in: .rect(cornerRadius: 11))
                .padding(.bottom, 2)

            Text(valeur)
                .font(.system(size: 38, weight: .bold, design: .rounded))
                .foregroundStyle(Color.vert)
                .contentTransition(.numericText())
            Text(legende)
                .font(.subheadline)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity, minHeight: 108)
        .padding(.horizontal, 8)
        .background {
            RoundedRectangle(cornerRadius: rayonHemiFit)
                .fill(.background.secondary)
                .shadow(color: .black.opacity(0.05), radius: 10, y: 4)
        }
    }

    private var carteSemaine: some View {
        let n = Statistiques.seancesSur7Jours(journal)
        return VStack(alignment: .leading, spacing: 12) {
            Text("Cette semaine")
                .font(.headline)
            Text(n > 0
                 ? "\(n) \(n == 1 ? "séance" : "séances") sur les 7 derniers jours."
                 : "Aucune séance ces 7 derniers jours. La prochaine vous attend, tranquillement.")
                .foregroundStyle(.secondary)

            HStack(spacing: 6) {
                ForEach(Statistiques.derniers7Jours(journal)) { jour in
                    VStack(spacing: 6) {
                        RoundedRectangle(cornerRadius: 12)
                            .fill(jour.actif ? AnyShapeStyle(.degradeAccent)
                                             : AnyShapeStyle(.quaternary))
                            .aspectRatio(1, contentMode: .fit)
                            .overlay {
                                if jour.actif {
                                    Image(systemName: "checkmark")
                                        .font(.subheadline.weight(.bold))
                                        .foregroundStyle(.white)
                                }
                            }
                        Text(jour.etiquette)
                            .font(.caption2.weight(.semibold))
                            .foregroundStyle(.secondary)
                    }
                }
            }
        }
        .carteHemiFit()
    }

    private var carteRienNeSePerd: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text("Rien de tout cela ne se perd")
                .font(.headline)
            Text("Ces minutes sont du travail réel accompli par votre cerveau. Une pause, même de plusieurs mois, ne les efface pas : vous reprendrez là où vous en êtes, jamais à zéro.")
                .foregroundStyle(.secondary)
        }
        .carteHemiFit()
    }

    private var dernieresSeances: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Dernières séances")
                .font(.title3.bold())

            if journal.isEmpty {
                Text("Aucune séance pour l'instant. La première est la plus importante, et elle vous attend sur l'accueil.")
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
                    Text("\(seance.minutes) min")
                        .foregroundStyle(.secondary)
                        .monospacedDigit()
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
