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
                        titre: "Il n'est jamais trop tard pour progresser",
                        texte: "On a longtemps cru que tout se jouait dans les six premiers mois. Cette idée a été largement remise en cause : le cerveau reste capable de créer de nouveaux chemins pendant des années, et des progrès ont été observés très longtemps après la lésion, chez des personnes qui continuaient à s'entraîner régulièrement. Cela demande de la patience, et les progrès sont souvent lents et partiels — mais ce qui compte n'est pas le temps écoulé depuis la lésion : c'est ce que vous faites à partir d'aujourd'hui."
                    )
                    conseil(
                        titre: "La sécurité avant tout",
                        texte: "Votre équilibre étant très altéré, aucun exercice de cette application ne se fait debout. Tout est prévu assis avec le dos soutenu, ou allongé. Bloquez toujours les freins du fauteuil avant de commencer, gardez une amplitude modérée, et ne tentez jamais un transfert ou un redressement seul si vous n'en êtes pas certain."
                    )
                    conseil(
                        titre: "Le pied en varus équin : ce qui se passe",
                        texte: "Deux choses s'additionnent. L'équin vient du mollet, devenu court et spastique, qui tire le pied en pointe. Le varus vient d'un muscle profond du bord interne de la jambe, le tibial postérieur, qui fait tourner la plante vers l'intérieur. En face, les muscles du bord externe du pied, ceux qui devraient le ramener à plat, sont affaiblis. D'où la règle : on détend et on étire le mollet et le bord interne, on réveille et on sollicite le bord externe. Massez avant d'étirer, étirez longtemps, et n'oubliez jamais la rotation vers l'extérieur — remonter le pied sans le tourner ne corrige que la moitié du problème."
                    )
                    conseil(
                        titre: "Vos attelles : les règles d'or",
                        texte: "Préparez toujours le pied avant de mettre l'attelle de nuit : massage puis étirement. Une attelle posée sur un pied froid et raide se supporte mal et se retire au bout d'une heure. Augmentez le temps de port progressivement, une à deux heures les premiers soirs, puis davantage sur une à deux semaines. Ne forcez jamais le pied dedans : s'il ne rentre pas, c'est qu'il faut étirer plus longtemps avant. Et surtout, contrôlez la peau à chaque retrait — talon, malléoles, bord externe du pied. Une rougeur qui persiste plus de vingt à trente minutes n'est pas normale : on arrête et on le signale."
                    )
                    conseil(
                        titre: "Attelle de jour : à faire préciser par vos soignants",
                        texte: "Une attelle qui exige d'être pieds nus et un releveur dynamique fixé sur la chaussure ne font pas le même travail. Le releveur dynamique a été conçu pour empêcher le pied de traîner pendant la marche ; en fauteuil, son intérêt tient surtout au maintien du pied en bonne position sur le repose-pied, et sa correction du varus reste limitée. Demandez à votre médecin de rééducation ou à votre orthoprothésiste laquelle des deux convient le mieux à vos journées assises, et combien d'heures la porter. C'est une vraie question, et vous avez le droit d'y avoir une réponse claire."
                    )
                    conseil(
                        titre: "Ce qui existe aussi contre le varus équin",
                        texte: "Au-delà des étirements et des attelles, il existe des traitements qui agissent sur la cause. Les injections de toxine botulique dans le mollet et le tibial postérieur relâchent précisément les muscles fautifs pendant plusieurs mois, ce qui rend ensuite les étirements et l'attelle bien plus efficaces. Les plâtres ou attelles de posture successifs permettent parfois de regagner de l'amplitude par étapes. Ce sont des options courantes et bien codifiées : parlez-en à votre médecin de médecine physique et de réadaptation."
                    )
                    conseil(
                        titre: "Le pied sur le repose-pied",
                        texte: "En varus, le pied ne repose pas à plat : il porte sur son bord externe, souvent sur la même petite zone toute la journée. Vérifiez la hauteur du repose-pied pour que la cheville soit le plus près possible de l'angle droit, et que l'appui se répartisse sur toute la plante. Un rembourrage souple sur le repose-pied aide. Regardez régulièrement ce bord externe : c'est un endroit où les rougeurs passent facilement inaperçues."
                    )
                    conseil(
                        titre: "Pourquoi masser avant de bouger",
                        texte: "Le massage fait baisser le tonus des muscles spastiques, réchauffe les tissus et réveille les sensations. Un membre massé s'étire beaucoup mieux : c'est pour cela que chaque séance commence par là. Vous pouvez masser autant de fois par jour que vous le souhaitez, il n'y a aucun risque à en faire trop, tant que c'est doux."
                    )
                    conseil(
                        titre: "Le soir, l'aide d'une tierce personne change tout",
                        texte: "Certaines mobilisations sont impossibles à faire seul : l'épaule, la hanche, l'étirement du mollet. Confiées le soir à une personne qui vous accompagne, elles entretiennent les articulations et prolongent leur effet pendant la nuit. La séance du soir de l'application est écrite pour être suivie par cette personne, consigne par consigne."
                    )
                    conseil(
                        titre: "Soulager les appuis, tout au long de la journée",
                        texte: "Rester assis longtemps met en tension les mêmes points d'appui. Prenez l'habitude de décharger vos appuis quelques secondes toutes les vingt à trente minutes, en vous penchant légèrement d'un côté puis de l'autre, mains sur les accoudoirs. C'est court, discret, et cela prévient les rougeurs et les escarres."
                    )
                    conseil(
                        titre: "Fermer facile, ouvrir difficile : c'est classique",
                        texte: "Après une lésion cérébrale, les muscles qui ferment la main restent forts (et spastiques), tandis que ceux qui l'ouvrent sont affaiblis. L'ouverture se rééduque donc avec de l'aide : le poignet plié vers l'avant desserre naturellement les doigts, la main gauche termine le mouvement, et chaque intention d'ouvrir — même sans mouvement visible — entraîne le cerveau."
                    )
                    conseil(
                        titre: "Comprendre la spasticité",
                        texte: "Vos muscles droits sont trop « toniques » : ils se contractent tout seuls et résistent, surtout quand on les étire vite. Ce n'est pas de la mauvaise volonté de votre main — c'est un réflexe. La bonne nouvelle : la lenteur, le calme et les étirements prolongés la font baisser."
                    )
                    conseil(
                        titre: "Lent, toujours plus lent",
                        texte: "Un mouvement rapide ou forcé déclenche le réflexe spastique : la main se referme encore plus. Étirez très lentement, arrêtez-vous dès que ça résiste, respirez… et attendez que ça lâche tout seul. Ça vient toujours."
                    )
                    conseil(
                        titre: "La chaleur détend",
                        texte: "La spasticité diminue avec la chaleur : faites les exercices de la main après une douche chaude, ou passez la main droite quelques minutes sous l'eau chaude (testez la température avec la main gauche). Le froid, le stress et la fatigue, eux, l'augmentent."
                    )
                    conseil(
                        titre: "Le cerveau apprend par la répétition",
                        texte: "Après une lésion cérébrale, le cerveau peut créer de nouveaux chemins : c'est la neuroplasticité. Elle se nourrit de répétitions courtes et fréquentes — 15 minutes par jour valent mieux qu'une heure une fois par semaine."
                    )
                    conseil(
                        titre: "Regardez votre côté droit",
                        texte: "Pendant les exercices, regardez votre main ou votre jambe droite bouger, même quand c'est la main gauche qui aide. Voir le mouvement aide le cerveau à le réapprendre."
                    )
                    conseil(
                        titre: "L'intention compte déjà",
                        texte: "Même si le mouvement ne vient pas, le fait d'essayer, d'imaginer et de vouloir bouger active les bonnes zones du cerveau. Aucun essai n'est perdu."
                    )
                    conseil(
                        titre: "Parlez de votre spasticité à vos soignants",
                        texte: "Il existe des traitements spécifiques de la spasticité (kinésithérapie, médicaments, injections ciblées, attelles) qui complètent très bien ces exercices. Si la spasticité vous gêne beaucoup, c'est une vraie question à poser à votre médecin."
                    )
                    conseil(
                        titre: "Les signaux pour s'arrêter",
                        texte: "Douleur vive, vertige, essoufflement inhabituel, fatigue soudaine, rougeur qui ne s'efface pas sur un point d'appui : on s'arrête, on se repose, et on en parle à son médecin si cela se répète."
                    )

                    Label {
                        Text("Ces exercices sont doux et classiques en rééducation, mais chaque situation est unique : faites-les valider par votre kinésithérapeute ou votre médecin, et signalez-leur toute douleur ou changement.")
                    } icon: {
                        Image(systemName: "info.circle")
                    }
                        .font(.subheadline)
                        .padding()
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .background(.yellow.opacity(0.15), in: .rect(cornerRadius: rayonHemiFit))
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
            .tint(.ardoise)
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
