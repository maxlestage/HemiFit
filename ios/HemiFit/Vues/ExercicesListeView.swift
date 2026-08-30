//
//  ExercicesListeView.swift
//  HemiFit
//

import SwiftUI

struct ExercicesListeView: View {
    @State private var seanceLibre: Seance?
    @State private var filtre: Filtre = .tous

    /// Filtre sur qui réalise l'exercice.
    enum Filtre: Hashable {
        case tous
        case mode(Realisation)

        var libelle: String {
            switch self {
            case .tous: "Tous"
            case .mode(let r): r.court
            }
        }
    }

    private func exercices(_ categorie: CategorieExercice) -> [Exercice] {
        Catalogue.parCategorie(categorie).filter { exercice in
            switch filtre {
            case .tous: true
            case .mode(let r): exercice.realisation == r
            }
        }
    }

    var body: some View {
        NavigationStack {
            List {
                Section {
                    Picker("Filtrer les exercices", selection: $filtre) {
                        Text("Tous").tag(Filtre.tous)
                        Text(Realisation.autonome.court)
                            .tag(Filtre.mode(.autonome))
                        Text(Realisation.tiercePersonne.court)
                            .tag(Filtre.mode(.tiercePersonne))
                    }
                    .pickerStyle(.segmented)
                    .listRowBackground(Color.clear)
                    .listRowInsets(.init(top: 4, leading: 0, bottom: 8, trailing: 0))
                }

                ForEach(CategorieExercice.allCases) { categorie in
                    let liste = exercices(categorie)
                    if !liste.isEmpty {
                        Section {
                            ForEach(liste) { exercice in
                                NavigationLink(value: exercice) {
                                    ligne(exercice)
                                }
                            }
                        } header: {
                            Label(categorie.titre, systemImage: categorie.symbole)
                                .font(.subheadline.weight(.semibold))
                        }
                    }
                }
            }
            .navigationTitle("Tous les exercices")
            .navigationDestination(for: Exercice.self) { exercice in
                DetailExerciceView(exercice: exercice) {
                    seanceLibre = Seance(
                        titre: exercice.nom,
                        description: "Exercice à la carte",
                        exercices: [exercice],
                        realisation: exercice.realisation
                    )
                }
            }
            .fullScreenCover(item: $seanceLibre) { seance in
                SeanceGuideeView(seance: seance)
            }
        }
    }

    private func ligne(_ exercice: Exercice) -> some View {
        HStack(spacing: 14) {
            Image(systemName: exercice.categorie.symbole)
                .font(.headline)
                .foregroundStyle(.secondary)
                .frame(width: 42, height: 42)
                .background(Color(.tertiarySystemFill), in: .rect(cornerRadius: 11))

            VStack(alignment: .leading, spacing: 2) {
                Text(exercice.nom)
                    .font(.headline)
                Text("\(exercice.dosage) · \(exercice.position.rawValue)")
                    .font(.subheadline)
                    .foregroundStyle(.secondary)
            }

            Spacer(minLength: 6)

            if exercice.realisation == .tiercePersonne {
                Image(systemName: Realisation.tiercePersonne.symbole)
                    .font(.subheadline.weight(.semibold))
                    .foregroundStyle(Color.aide)
                    .frame(width: 30, height: 30)
                    .background(Color.aideClair, in: .rect(cornerRadius: 9))
                    .accessibilityLabel("Avec une tierce personne")
            }
        }
        .padding(.vertical, 8)
    }
}

struct DetailExerciceView: View {
    let exercice: Exercice
    let onLancer: () -> Void

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 14) {
                HStack(spacing: 8) {
                    Label(
                        "\(exercice.categorie.titre) · \(exercice.position.rawValue)",
                        systemImage: exercice.categorie.symbole
                    )
                    .font(.subheadline.weight(.semibold))
                    .padding(.horizontal, 14)
                    .padding(.vertical, 6)
                    .background(Color.vertClair, in: .capsule)
                    .foregroundStyle(Color.vert)

                    PastilleRealisation(realisation: exercice.realisation)
                }

                Text(exercice.objectif)
                    .foregroundStyle(.secondary)

                Text(exercice.dosage)
                    .font(.headline)

                ForEach(exercice.etapes, id: \.self) { etape in
                    HStack(alignment: .top, spacing: 10) {
                        Circle()
                            .fill(Color.vert)
                            .frame(width: 7, height: 7)
                            .padding(.top, 9)
                        Text(etape)
                    }
                    .padding(.vertical, 4)
                }

                Button {
                    onLancer()
                } label: {
                    Label("Faire cet exercice", systemImage: "play.fill")
                }
                .buttonStyle(BoutonLargeStyle())
            }
            .padding()
        }
        .navigationTitle(exercice.nom)
        .navigationBarTitleDisplayMode(.large)
    }
}

#Preview {
    ExercicesListeView()
}
