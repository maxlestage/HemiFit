//
//  ExercicesListeView.swift
//  HemiFit
//

import SwiftUI

struct ExercicesListeView: View {
    @State private var seanceLibre: Seance?
    @State private var filtre: Filtre = .tous
    /// Famille mise en avant ; `nil` affiche tout le catalogue.
    @State private var famille: CategorieExercice?

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

    /// Familles affichées : une seule si l'on en a choisi une, sinon toutes.
    private var famillesAffichees: [CategorieExercice] {
        if let famille { [famille] } else { CategorieExercice.allCases }
    }

    private var listeVide: Bool {
        famillesAffichees.allSatisfy { exercices($0).isEmpty }
    }

    var body: some View {
        NavigationStack {
            List {
                Section {
                    pastillesFamilles
                        .listRowBackground(Color.clear)
                        .listRowInsets(
                            .init(top: 4, leading: 0, bottom: 4, trailing: 0)
                        )

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

                ForEach(famillesAffichees) { categorie in
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

                if listeVide {
                    Section {
                        Text(messageListeVide)
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                            .multilineTextAlignment(.center)
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 12)
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

    /// Une pastille par famille, toutes visibles d'un coup d'œil : le
    /// catalogue est long, et faire défiler coûte cher quand on n'a
    /// qu'une main. Les pastilles passent à la ligne plutôt que de
    /// défiler, pour qu'aucune ne se cache hors écran.
    private var pastillesFamilles: some View {
        LazyVGrid(
            columns: [GridItem(.adaptive(minimum: 108), spacing: 6)],
            spacing: 6
        ) {
            pastille(titre: "Toutes", symbole: nil, choisie: famille == nil) {
                famille = nil
            }
            ForEach(CategorieExercice.allCases) { categorie in
                pastille(
                    titre: categorie.court,
                    symbole: categorie.symbole,
                    choisie: famille == categorie
                ) {
                    famille = famille == categorie ? nil : categorie
                }
            }
        }
    }

    private func pastille(
        titre: String,
        symbole: String?,
        choisie: Bool,
        action: @escaping () -> Void
    ) -> some View {
        Button(action: action) {
            HStack(spacing: 6) {
                if let symbole {
                    Image(systemName: symbole)
                        .font(.footnote.weight(.semibold))
                }
                Text(titre)
                    .font(.subheadline.weight(.semibold))
                    .lineLimit(1)
            }
            .frame(maxWidth: .infinity, minHeight: 50)
            .padding(.horizontal, 10)
            .background(
                choisie ? Color.ardoise : Color(.secondarySystemGroupedBackground),
                in: .capsule
            )
            .foregroundStyle(choisie ? Color.white : Color.secondary)
            .overlay(
                Capsule().stroke(
                    choisie ? Color.clear : Color(.separator),
                    lineWidth: 1
                )
            )
        }
        .buttonStyle(.plain)
        .accessibilityAddTraits(choisie ? [.isSelected] : [])
    }

    private var messageListeVide: String {
        switch filtre {
        case .tous:
            "Aucun exercice dans cette famille."
        case .mode(.autonome):
            """
            Aucun exercice de cette famille ne se fait en autonomie. \
            Touchez « Toutes » pour revoir l'ensemble du catalogue.
            """
        case .mode(.tiercePersonne):
            """
            Aucun exercice de cette famille ne se fait avec une tierce \
            personne. Touchez « Toutes » pour revoir l'ensemble du catalogue.
            """
        }
    }

    private func ligne(_ exercice: Exercice) -> some View {
        HStack(spacing: 14) {
            Image(systemName: exercice.categorie.symbole)
                .font(.headline)
                .foregroundStyle(.secondary)
                .frame(width: 42, height: 42)
                .background(Color(.tertiarySystemFill), in: .rect(cornerRadius: 8))

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
                    .background(Color.aideClair, in: .rect(cornerRadius: 6))
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
                    .background(Color.ardoiseClaire, in: .capsule)
                    .foregroundStyle(Color.ardoise)

                    PastilleRealisation(realisation: exercice.realisation)
                }

                Text(exercice.objectif)
                    .foregroundStyle(.secondary)

                Text(exercice.dosage)
                    .font(.headline)

                ForEach(exercice.etapes, id: \.self) { etape in
                    HStack(alignment: .top, spacing: 10) {
                        Circle()
                            .fill(Color.ardoise)
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
