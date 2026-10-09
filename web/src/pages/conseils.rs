//! Conseils : quelques repères pour une rééducation sereine.

use yew::prelude::*;

use crate::composants::{BandeauSecurite, TitreFente};

struct Conseil {
    titre: &'static str,
    texte: &'static str,
}

const CONSEILS: &[Conseil] = &[
    Conseil {
        titre: "Il n'est jamais trop tard pour progresser",
        texte: "On a longtemps cru que tout se jouait dans les six premiers mois. Cette idée a été largement remise en cause : le cerveau reste capable de créer de nouveaux chemins pendant des années, et des progrès ont été observés très longtemps après la lésion, chez des personnes qui continuaient à s'entraîner régulièrement. Cela demande de la patience, et les progrès sont souvent lents et partiels, mais ce qui compte n'est pas le temps écoulé depuis la lésion : c'est ce que vous faites à partir d'aujourd'hui.",
    },
    Conseil {
        titre: "La sécurité avant tout",
        texte: "Votre équilibre étant très altéré, aucun exercice de cette application ne se fait debout. Tout est prévu assis avec le dos soutenu, ou allongé. Bloquez toujours les freins du fauteuil avant de commencer, gardez une amplitude modérée, et ne tentez jamais un transfert ou un redressement seul si vous n'en êtes pas certain.",
    },
    Conseil {
        titre: "Le pied en varus équin : ce qui se passe",
        texte: "Deux choses s'additionnent. L'équin vient du mollet, devenu court et spastique, qui tire le pied en pointe. Le varus vient d'un muscle profond du bord interne de la jambe, le tibial postérieur, qui fait tourner la plante vers l'intérieur. En face, les muscles du bord externe du pied, ceux qui devraient le ramener à plat, sont affaiblis. D'où la règle : on détend et on étire le mollet et le bord interne, on réveille et on sollicite le bord externe. Massez avant d'étirer, étirez longtemps, et n'oubliez jamais la rotation vers l'extérieur — remonter le pied sans le tourner ne corrige que la moitié du problème.",
    },
    Conseil {
        titre: "Vos attelles : les règles d'or",
        texte: "Préparez toujours le pied avant de mettre l'attelle de nuit : massage puis étirement. Une attelle posée sur un pied froid et raide se supporte mal et se retire au bout d'une heure. Augmentez le temps de port progressivement, une à deux heures les premiers soirs, puis davantage sur une à deux semaines. Ne forcez jamais le pied dedans : s'il ne rentre pas, c'est qu'il faut étirer plus longtemps avant. Et surtout, contrôlez la peau à chaque retrait — talon, malléoles, bord externe du pied. Une rougeur qui persiste plus de vingt à trente minutes n'est pas normale : on arrête et on le signale.",
    },
    Conseil {
        titre: "Attelle de jour : à faire préciser par vos soignants",
        texte: "Une attelle qui exige d'être pieds nus et un releveur dynamique fixé sur la chaussure ne font pas le même travail. Le releveur dynamique a été conçu pour empêcher le pied de traîner pendant la marche ; en fauteuil, son intérêt tient surtout au maintien du pied en bonne position sur le repose-pied, et sa correction du varus reste limitée. Demandez à votre médecin de rééducation ou à votre orthoprothésiste laquelle des deux convient le mieux à vos journées assises, et combien d'heures la porter. C'est une vraie question, et vous avez le droit d'y avoir une réponse claire.",
    },
    Conseil {
        titre: "Ce qui existe aussi contre le varus équin",
        texte: "Au-delà des étirements et des attelles, il existe des traitements qui agissent sur la cause. Les injections de toxine botulique dans le mollet et le tibial postérieur relâchent précisément les muscles fautifs pendant plusieurs mois, ce qui rend ensuite les étirements et l'attelle bien plus efficaces. Les plâtres ou attelles de posture successifs permettent parfois de regagner de l'amplitude par étapes. Ce sont des options courantes et bien codifiées : parlez-en à votre médecin de médecine physique et de réadaptation.",
    },
    Conseil {
        titre: "Le pied sur le repose-pied",
        texte: "En varus, le pied ne repose pas à plat : il porte sur son bord externe, souvent sur la même petite zone toute la journée. Vérifiez la hauteur du repose-pied pour que la cheville soit le plus près possible de l'angle droit, et que l'appui se répartisse sur toute la plante. Un rembourrage souple sur le repose-pied aide. Regardez régulièrement ce bord externe : c'est un endroit où les rougeurs passent facilement inaperçues.",
    },
    Conseil {
        titre: "Électrodes : quels muscles stimuler",
        texte: "La règle est simple et elle est contre-intuitive : on ne stimule jamais le muscle trop contracté, mais celui qui lui fait face et qui manque de force. Pour la main, on place donc les électrodes sur le DESSUS de l'avant-bras, sur les muscles qui ouvrent les doigts — jamais côté paume. Pour le pied, on les place sur le devant de la jambe, à l'extérieur de l'arête du tibia, sur les muscles qui relèvent le pied — jamais sur le mollet. Stimuler ces muscles affaiblis les renforce, et fait baisser en retour le tonus du muscle spastique d'en face. Le bon repère est visuel : la main doit s'ouvrir, le pied doit se relever. Si c'est l'inverse, les électrodes sont du mauvais côté.",
    },
    Conseil {
        titre: "Électrodes : le geste qui change tout",
        texte: "Au moment où le courant monte, essayez d'ouvrir la main, ou de relever le pied, par vous-même, en même temps. C'est l'association de la stimulation et de votre propre intention qui fait progresser la commande motrice. La stimulation subie passivement entretient le muscle, ce qui est déjà utile, mais elle réapprend beaucoup moins au cerveau à commander le mouvement. Faites les séances après le massage : un muscle réchauffé et détendu répond mieux.",
    },
    Conseil {
        titre: "Électrodes : les précautions à connaître",
        texte: "Trois situations imposent un avis médical avant toute utilisation : un stimulateur cardiaque ou un défibrillateur implanté, un antécédent de crise d'épilepsie — question qui se pose après une lésion cérébrale — et une peau dont la sensibilité est diminuée, car on ne sent alors pas venir la brûlure. Ne montez jamais l'intensité simplement pour « sentir quelque chose ». Jamais d'électrodes sur une peau abîmée, irritée ou blessée, ni sur l'avant du cou. Éteignez toujours l'appareil avant de décoller les électrodes, et remplacez-les dès qu'elles n'adhèrent plus partout. Enfin, les réglages de l'appareil — fréquence, largeur d'impulsion, temps de montée, durée de contraction et de repos — doivent être fixés par votre kinésithérapeute ou votre médecin, pas réglés au hasard.",
    },
    Conseil {
        titre: "Comment on regagne vraiment du muscle",
        texte: "Le muscle ne se reconstruit pas en faisant un peu de tout tous les jours. Il lui faut trois choses. Un effort suffisant, d'abord : les dernières répétitions d'une série doivent être difficiles, sinon le muscle n'a aucune raison de changer. De la régularité ensuite : deux à trois séances par semaine pour un même groupe musculaire, pas davantage. Et surtout du repos — quarante-huit heures entre deux séances qui sollicitent les mêmes muscles, car c'est pendant le repos que le muscle se construit, jamais pendant l'effort. N'augmentez la difficulté que lorsque la dernière répétition devient facile.",
    },
    Conseil {
        titre: "Protégez votre épaule gauche",
        texte: "Votre épaule gauche fait tout : les transferts, les poussées, l'assistance au côté droit. C'est l'articulation la plus exposée à l'usure chez les personnes en fauteuil, et une épaule gauche douloureuse coûterait bien plus cher que tout ce que l'on peut gagner ailleurs. C'est la raison d'être de l'exercice des rotateurs, à ne jamais sauter : il renforce les petits muscles profonds qui maintiennent l'épaule en place. Résistance légère, amplitude modérée — ce n'est pas là qu'il faut forcer.",
    },
    Conseil {
        titre: "Ne bloquez jamais votre respiration",
        texte: "Retenir son souffle pendant un effort fait grimper brutalement la tension artérielle. Soufflez pendant la phase difficile, inspirez pendant le retour. Un repère simple : si vous ne pouvez pas parler pendant l'exercice, c'est que vous bloquez votre respiration ou que l'effort est trop intense.",
    },
    Conseil {
        titre: "Sans protéines, l'entraînement ne donne rien",
        texte: "On ne reconstruit pas du muscle sans matériau. Après une longue période d'immobilité, l'apport en protéines est souvent devenu insuffisant, et l'entraînement seul n'apporte alors presque rien. Mieux vaut les répartir sur la journée qu'en un seul repas. Demandez à votre médecin de vérifier vos apports, et au besoin de vous orienter vers un diététicien : c'est une consultation qui change les résultats.",
    },
    Conseil {
        titre: "À quoi s'attendre, honnêtement",
        texte: "Les deux côtés ne progresseront pas au même rythme. Le côté gauche, simplement désentraîné, peut regagner de la force en quelques semaines — vous le sentirez d'abord dans vos transferts. Le côté droit, où la commande nerveuse est abîmée, progresse beaucoup plus lentement, et parfois très peu en force pure. Ce n'est pas un échec : de ce côté, l'objectif est d'entretenir le muscle, de garder la commande vivante et d'empêcher que la situation ne se dégrade. Les deux comptent, et le premier rend le second possible.",
    },
    Conseil {
        titre: "Le matériel, du plus simple au plus utile",
        texte: "Commencez sans rien : le poids de votre bras, la résistance de votre propre main, les accoudoirs du fauteuil. Ajoutez ensuite un jeu d'élastiques de résistances différentes — c'est le matériel le plus utile en fauteuil : léger, progressif, et sans danger s'il vous échappe. Les bracelets lestés et les petits haltères viennent après. Évitez tout ce qui doit être soulevé au-dessus de la tête tant que votre équilibre assis n'est pas parfaitement sûr.",
    },
    Conseil {
        titre: "Courbatures ou douleur ?",
        texte: "Une courbature diffuse, qui apparaît le lendemain et s'estompe en deux ou trois jours, est normale et sans gravité. Une douleur vive pendant l'effort, une douleur d'articulation, un gonflement ou une rougeur ne le sont pas : on arrête et on en parle. Et si la spasticité augmente nettement dans les heures qui suivent une séance, c'est que l'effort était trop intense ou trop rapide — réduisez la charge et ralentissez le mouvement.",
    },
    Conseil {
        titre: "Pourquoi masser avant de bouger",
        texte: "Le massage fait baisser le tonus des muscles spastiques, réchauffe les tissus et réveille les sensations. Un membre massé s'étire beaucoup mieux : c'est pour cela que chaque séance commence par là. Vous pouvez masser autant de fois par jour que vous le souhaitez, il n'y a aucun risque à en faire trop tant que c'est doux.",
    },
    Conseil {
        titre: "Le soir, l'aide d'une tierce personne change tout",
        texte: "Certaines mobilisations sont impossibles à faire seul : l'épaule, la hanche, l'étirement du mollet. Confiées le soir à une personne qui vous accompagne, elles entretiennent les articulations et prolongent leur effet pendant la nuit. La séance du soir de l'application est écrite pour être suivie par cette personne, consigne par consigne.",
    },
    Conseil {
        titre: "Soulager les appuis, tout au long de la journée",
        texte: "Rester assis longtemps met en tension les mêmes points d'appui. Prenez l'habitude de décharger vos appuis quelques secondes toutes les vingt à trente minutes, en vous penchant légèrement d'un côté puis de l'autre, mains sur les accoudoirs. C'est court, discret, et cela prévient les rougeurs et les escarres.",
    },
    Conseil {
        titre: "Fermer facile, ouvrir difficile : c'est classique",
        texte: "Après une lésion cérébrale, les muscles qui ferment la main restent forts et spastiques, tandis que ceux qui l'ouvrent sont affaiblis. L'ouverture se rééduque donc avec de l'aide : le poignet plié vers l'avant desserre naturellement les doigts, la main gauche termine le mouvement, et chaque intention d'ouvrir, même sans mouvement visible, entraîne le cerveau.",
    },
    Conseil {
        titre: "Comprendre la spasticité",
        texte: "Vos muscles droits sont trop toniques : ils se contractent seuls et résistent, surtout quand on les étire vite. Ce n'est pas de la mauvaise volonté de votre main, c'est un réflexe. La bonne nouvelle : la lenteur, le calme et les étirements prolongés le font baisser.",
    },
    Conseil {
        titre: "Lent, toujours plus lent",
        texte: "Un mouvement rapide ou forcé déclenche le réflexe spastique et la main se referme encore plus. Étirez très lentement, arrêtez-vous dès que ça résiste, respirez, et attendez que cela lâche de soi-même.",
    },
    Conseil {
        titre: "La chaleur détend",
        texte: "La spasticité diminue avec la chaleur : faites les exercices de la main après une douche chaude, ou passez la main droite quelques minutes sous l'eau chaude en testant la température avec la main gauche. Le froid, le stress et la fatigue, eux, l'augmentent.",
    },
    Conseil {
        titre: "Le cerveau apprend par la répétition",
        texte: "La neuroplasticité se nourrit de répétitions courtes et fréquentes. Quinze minutes chaque jour valent mieux qu'une heure une fois par semaine.",
    },
    Conseil {
        titre: "Regardez votre côté droit",
        texte: "Pendant les exercices, regardez votre main ou votre jambe droite bouger, même quand c'est la main gauche qui fait le travail. Voir le mouvement aide le cerveau à le réapprendre.",
    },
    Conseil {
        titre: "L'intention compte déjà",
        texte: "Même si le mouvement ne vient pas, le fait d'essayer, d'imaginer et de vouloir bouger active les bonnes zones du cerveau. Aucun essai n'est perdu.",
    },
    Conseil {
        titre: "Parlez de votre spasticité à vos soignants",
        texte: "Il existe des traitements spécifiques de la spasticité : kinésithérapie, médicaments, injections ciblées, attelles de posture. Ils complètent très bien ces exercices. Si la spasticité vous gêne beaucoup, c'est une vraie question à poser à votre médecin.",
    },
    Conseil {
        titre: "Les signaux pour s'arrêter",
        texte: "Douleur vive, vertige, essoufflement inhabituel, fatigue soudaine, rougeur qui ne s'efface pas sur un point d'appui : on s'arrête, on se repose, et on en parle à son médecin si cela se répète.",
    },
];

#[component]
pub fn Conseils() -> Html {
    html! {
        <>
            <header class="entete">
                <p class="surtitre-page">{ format!("{} repères", CONSEILS.len()) }</p>
                <TitreFente texte="Conseils" />
                <p class="chapo">{ "Quelques repères pour une rééducation sereine." }</p>
            </header>

            for (rang, conseil) in CONSEILS.iter().enumerate() {
                <article class="carte conseil" data-revele="">
                    <span class="numero">{ format!("{:02}", rang + 1) }</span>
                    <h3>{ conseil.titre }</h3>
                    <p>{ conseil.texte }</p>
                </article>
            }

            <BandeauSecurite texte="Ces exercices sont doux et classiques en rééducation, mais chaque \
                situation est unique : faites-les valider par votre kinésithérapeute ou votre \
                médecin, et signalez-leur toute douleur ou changement." />
        </>
    }
}
