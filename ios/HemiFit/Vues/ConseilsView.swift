//
//  ConseilsView.swift
//  HemiFit
//

import SwiftUI

struct ConseilsView: View {
    @AppStorage("rappelActif") private var rappelActif = false
    @AppStorage("rappelHeure") private var rappelHeure = 10
    @AppStorage("rappelMinute") private var rappelMinute = 0

    @State private var rappels = Rappels()

    private var heureRappel: Binding<Date> {
        Binding {
            Calendar.current.date(
                bySettingHour: rappelHeure, minute: rappelMinute, second: 0, of: .now
            ) ?? .now
        } set: { nouvelle in
            let composants = Calendar.current.dateComponents([.hour, .minute], from: nouvelle)
            rappelHeure = composants.hour ?? 10
            rappelMinute = composants.minute ?? 0
            if rappelActif {
                Task { await rappels.programmer(heure: rappelHeure, minute: rappelMinute) }
            }
        }
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 16) {
                    carteRappel

                    conseil(
                        titre: "🐢 La lenteur est votre alliée",
                        texte: "Un mouvement lent et contrôlé fait plus travailler le cerveau qu'un mouvement rapide. Prenez votre temps, respirez."
                    )
                    conseil(
                        titre: "🧠 Le cerveau apprend par la répétition",
                        texte: "Après une lésion cérébrale, le cerveau peut créer de nouveaux chemins : c'est la neuroplasticité. Elle se nourrit de répétitions courtes et fréquentes — 15 minutes par jour valent mieux qu'une heure une fois par semaine."
                    )
                    conseil(
                        titre: "👀 Regardez votre côté droit",
                        texte: "Pendant les exercices, regardez votre main ou votre jambe droite bouger, même quand c'est la main gauche qui aide. Voir le mouvement aide le cerveau à le réapprendre."
                    )
                    conseil(
                        titre: "✋ L'intention compte déjà",
                        texte: "Même si le mouvement ne vient pas, le fait d'essayer, d'imaginer et de vouloir bouger active les bonnes zones du cerveau. Aucun essai n'est perdu."
                    )
                    conseil(
                        titre: "🛑 Les signaux pour s'arrêter",
                        texte: "Douleur vive, vertige, essoufflement inhabituel, fatigue soudaine : on s'arrête, on se repose, et on en parle à son médecin si ça se répète."
                    )

                    Text("⚕️ Ces exercices sont doux et classiques en rééducation, mais chaque situation est unique : faites-les valider par votre kinésithérapeute ou votre médecin, et signalez-leur toute douleur ou changement.")
                        .font(.subheadline)
                        .padding()
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .background(.yellow.opacity(0.15), in: .rect(cornerRadius: 20))
                }
                .padding()
            }
            .navigationTitle("Conseils")
        }
    }

    private var carteRappel: some View {
        VStack(alignment: .leading, spacing: 10) {
            Toggle(isOn: $rappelActif) {
                Label("Rappel quotidien", systemImage: "bell.fill")
                    .font(.headline)
            }
            .tint(.vert)
            .onChange(of: rappelActif) { _, actif in
                Task {
                    if actif {
                        await rappels.demanderAutorisation()
                        await rappels.programmer(heure: rappelHeure, minute: rappelMinute)
                    } else {
                        rappels.annuler()
                    }
                }
            }

            if rappelActif {
                DatePicker(
                    "Heure du rappel",
                    selection: heureRappel,
                    displayedComponents: .hourAndMinute
                )
                Text("Une petite notification chaque jour pour garder le rythme, en douceur.")
                    .font(.subheadline)
                    .foregroundStyle(.secondary)
            }
        }
        .carteHemiFit()
    }

    private func conseil(titre: String, texte: String) -> some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(titre)
                .font(.headline)
            Text(texte)
                .foregroundStyle(.secondary)
        }
        .carteHemiFit()
    }
}

#Preview {
    ConseilsView()
}
