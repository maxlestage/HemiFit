//
//  SeanceGuideeView.swift
//  HemiFit
//
//  Lecteur de séance : un exercice à la fois, un grand minuteur,
//  et des boutons larges en bas d'écran.
//

import SwiftData
import SwiftUI

struct SeanceGuideeView: View {
    let seance: Seance

    @Environment(\.dismiss) private var dismiss
    @State private var indice = 0
    @State private var terminee = false

    private var exercice: Exercice { seance.exercices[indice] }
    private var dernier: Bool { indice == seance.exercices.count - 1 }

    var body: some View {
        NavigationStack {
            if terminee {
                FinDeSeanceView(seance: seance)
            } else {
                VStack(alignment: .leading, spacing: 12) {
                    barreProgression

                    ScrollView {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Exercice \(indice + 1) sur \(seance.exercices.count)")
                                .foregroundStyle(.secondary)

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

                                if exercice.realisation == .tiercePersonne {
                                    PastilleRealisation(realisation: .tiercePersonne)
                                }
                            }

                            Text(exercice.nom)
                                .font(.title.bold())

                            Text(exercice.objectif)
                                .foregroundStyle(.secondary)

                            Text(exercice.dosage)
                                .font(.headline)

                            MinuteurView(dureeSec: exercice.dureeSec)
                                .id(exercice.id) // remis à zéro à chaque exercice
                                .frame(maxWidth: .infinity)

                            ForEach(exercice.etapes, id: \.self) { etape in
                                HStack(alignment: .top, spacing: 10) {
                                    Text("•")
                                        .foregroundStyle(Color.vert)
                                        .font(.title3)
                                    Text(etape)
                                }
                                .padding(.vertical, 4)
                            }
                        }
                    }

                    Button(dernier ? "Terminer la séance" : "Exercice suivant") {
                        if dernier {
                            terminee = true
                        } else {
                            indice += 1
                        }
                    }
                    .buttonStyle(BoutonLargeStyle())

                    Button("Arrêter la séance") {
                        dismiss()
                    }
                    .frame(maxWidth: .infinity, minHeight: 52)
                    .foregroundStyle(.secondary)
                }
                .padding()
            }
        }
        .interactiveDismissDisabled()
    }

    private var barreProgression: some View {
        HStack(spacing: 6) {
            ForEach(seance.exercices.indices, id: \.self) { i in
                Capsule()
                    .fill(i <= indice ? AnyShapeStyle(Color.vert)
                                      : AnyShapeStyle(.quaternary))
                    .frame(height: 4)
            }
        }
    }
}

// MARK: - Minuteur

struct MinuteurView: View {
    let dureeSec: Int

    @State private var restant: Int = 0
    @State private var enPause = false

    private let horloge = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

    /// Part du temps restant, pour la jauge circulaire.
    private var proportion: Double {
        dureeSec > 0 ? Double(restant) / Double(dureeSec) : 0
    }

    var body: some View {
        VStack(spacing: 16) {
            ZStack {
                Circle()
                    .stroke(.quaternary, lineWidth: 9)

                Circle()
                    .trim(from: 0, to: proportion)
                    .stroke(
                        Color.vert,
                        style: StrokeStyle(lineWidth: 9, lineCap: .round)
                    )
                    .rotationEffect(.degrees(-90))
                    .animation(.linear(duration: 1), value: restant)

                if restant > 0 {
                    Text(texteTemps)
                        .font(.system(size: 52, weight: .bold))
                        .monospacedDigit()
                        .foregroundStyle(.primary)
                        .contentTransition(.numericText())
                } else {
                    Text("Terminé")
                        .font(.title2.weight(.bold))
                        .foregroundStyle(Color.vert)
                        .multilineTextAlignment(.center)
                        .padding(.horizontal, 12)
                }
            }
            .frame(width: 208, height: 208)
            .accessibilityLabel(
                restant > 0 ? "Temps restant : \(texteTemps)" : "Exercice terminé"
            )

            if restant > 0 {
                Button {
                    enPause.toggle()
                } label: {
                    Label(enPause ? "Reprendre" : "Pause", systemImage: enPause ? "play.fill" : "pause.fill")
                }
                .buttonStyle(BoutonLargeStyle(degrade: false, couleurTexte: .vert))
            }
        }
        .onAppear { restant = dureeSec }
        .onReceive(horloge) { _ in
            guard !enPause, restant > 0 else { return }
            restant -= 1
        }
    }

    private var texteTemps: String {
        String(format: "%d:%02d", restant / 60, restant % 60)
    }
}

// MARK: - Fin de séance

struct FinDeSeanceView: View {
    let seance: Seance

    @Environment(\.dismiss) private var dismiss
    @Environment(\.modelContext) private var contexte
    @State private var ressenti: Ressenti?

    var body: some View {
        VStack(spacing: 16) {
            Spacer()

            Image(systemName: "checkmark")
                .font(.system(size: 36, weight: .bold))
                .foregroundStyle(.white)
                .frame(width: 76, height: 76)
                .background(Color.vert, in: .circle)
            Text("Séance terminée")
                .font(.largeTitle.bold())
            Text("Chaque séance renforce les nouveaux chemins de votre cerveau. Vous pouvez en être fier.")
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)

            Spacer()

            Text("Comment vous sentez-vous ?")
                .font(.headline)
                .frame(maxWidth: .infinity, alignment: .leading)

            HStack(spacing: 12) {
                ForEach(Ressenti.allCases) { choix in
                    Button {
                        ressenti = choix
                    } label: {
                        VStack(spacing: 10) {
                            // Jauge à trois barres, plus sobre qu'un émoticône.
                            HStack(alignment: .bottom, spacing: 4) {
                                ForEach(1...3, id: \.self) { n in
                                    Capsule()
                                        .fill(n <= choix.niveau
                                              ? AnyShapeStyle(ressenti == choix ? Color.vert : Color.secondary)
                                              : AnyShapeStyle(.quaternary))
                                        .frame(width: 7, height: CGFloat(6 + n * 5))
                                }
                            }
                            .frame(height: 22)

                            Text(choix.libelle).font(.subheadline.weight(.semibold))
                        }
                        .frame(maxWidth: .infinity, minHeight: 92)
                        .background(
                            ressenti == choix ? Color.vertClair : Color(.secondarySystemBackground),
                            in: .rect(cornerRadius: rayonHemiFit)
                        )
                        .overlay(
                            RoundedRectangle(cornerRadius: rayonHemiFit)
                                .stroke(ressenti == choix ? Color.vert : .clear, lineWidth: 2)
                        )
                        .offset(y: ressenti == choix ? -2 : 0)
                        .animation(.spring(duration: 0.25), value: ressenti)
                    }
                    .buttonStyle(.plain)
                }
            }

            Button("Enregistrer la séance") {
                contexte.insert(JournalSeance(
                    titre: seance.titre,
                    minutes: seance.dureeMinutes,
                    exercicesFaits: seance.exercices.count,
                    ressenti: ressenti
                ))
                dismiss()
            }
            .buttonStyle(BoutonLargeStyle())
        }
        .padding()
    }
}

#Preview {
    SeanceGuideeView(seance: Catalogue.seanceDuJour())
        .modelContainer(for: JournalSeance.self, inMemory: true)
}
