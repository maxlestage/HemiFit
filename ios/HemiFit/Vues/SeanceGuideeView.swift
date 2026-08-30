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

                            Text("\(exercice.categorie.emoji) \(exercice.categorie.titre) · \(exercice.position.rawValue)")
                                .font(.subheadline.weight(.semibold))
                                .padding(.horizontal, 14)
                                .padding(.vertical, 6)
                                .background(Color.vertClair, in: .capsule)
                                .foregroundStyle(Color.vert)

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

                    Button(dernier ? "Terminer la séance ✅" : "Exercice suivant →") {
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
                    .fill(i <= indice ? Color.vert : Color.secondary.opacity(0.25))
                    .frame(height: 8)
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

    var body: some View {
        VStack(spacing: 12) {
            Text(restant > 0 ? texteTemps : "Bien joué 💚")
                .font(.system(size: 56, weight: .bold, design: .rounded))
                .monospacedDigit()
                .foregroundStyle(Color.vert)
                .contentTransition(.numericText())

            if restant > 0 {
                Button {
                    enPause.toggle()
                } label: {
                    Label(enPause ? "Reprendre" : "Pause", systemImage: enPause ? "play.fill" : "pause.fill")
                }
                .buttonStyle(BoutonLargeStyle(couleurFond: .vertClair, couleurTexte: .vert))
            }
        }
        .onAppear { restant = dureeSec }
        .onReceive(horloge) { _ in
            guard !enPause, restant > 0 else { return }
            withAnimation { restant -= 1 }
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

            Text("🎉")
                .font(.system(size: 64))
            Text("Séance terminée !")
                .font(.largeTitle.bold())
            Text("Chaque séance renforce les nouveaux chemins de votre cerveau. Soyez fier de vous.")
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
                        VStack(spacing: 6) {
                            Text(choix.emoji).font(.title)
                            Text(choix.libelle).font(.subheadline.weight(.semibold))
                        }
                        .frame(maxWidth: .infinity, minHeight: 88)
                        .background(
                            ressenti == choix ? Color.vertClair : Color(.secondarySystemBackground),
                            in: .rect(cornerRadius: 20)
                        )
                        .overlay(
                            RoundedRectangle(cornerRadius: 20)
                                .stroke(ressenti == choix ? Color.vert : .clear, lineWidth: 2)
                        )
                    }
                    .buttonStyle(.plain)
                }
            }

            Button("Enregistrer ma séance 💾") {
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
