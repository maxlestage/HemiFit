//
//  ExercicesListeView.swift
//  HemiFit
//

import SwiftUI

struct ExercicesListeView: View {
    @State private var seanceLibre: Seance?

    var body: some View {
        NavigationStack {
            List {
                Section {
                    Text("Touchez un exercice pour voir les consignes, ou lancez-le seul quand vous en avez envie.")
                        .foregroundStyle(.secondary)
                        .listRowBackground(Color.clear)
                }

                ForEach(CategorieExercice.allCases) { categorie in
                    Section("\(categorie.emoji) \(categorie.titre)") {
                        ForEach(Catalogue.parCategorie(categorie)) { exercice in
                            NavigationLink(value: exercice) {
                                VStack(alignment: .leading, spacing: 2) {
                                    Text(exercice.nom)
                                        .font(.headline)
                                    Text("\(exercice.dosage) · \(exercice.position.rawValue)")
                                        .font(.subheadline)
                                        .foregroundStyle(.secondary)
                                }
                                .padding(.vertical, 8)
                            }
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
                        exercices: [exercice]
                    )
                }
            }
            .fullScreenCover(item: $seanceLibre) { seance in
                SeanceGuideeView(seance: seance)
            }
        }
    }
}

struct DetailExerciceView: View {
    let exercice: Exercice
    let onLancer: () -> Void

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 14) {
                Text("\(exercice.categorie.emoji) \(exercice.categorie.titre) · \(exercice.position.rawValue)")
                    .font(.subheadline.weight(.semibold))
                    .padding(.horizontal, 14)
                    .padding(.vertical, 6)
                    .background(Color.vertClair, in: .capsule)
                    .foregroundStyle(Color.vert)

                Text(exercice.objectif)
                    .foregroundStyle(.secondary)

                Text(exercice.dosage)
                    .font(.headline)

                ForEach(exercice.etapes, id: \.self) { etape in
                    HStack(alignment: .top, spacing: 10) {
                        Text("•")
                            .foregroundStyle(Color.vert)
                            .font(.title3)
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
