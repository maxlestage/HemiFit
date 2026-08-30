//
//  Modeles.swift
//  HemiFit
//
//  Modèle SwiftData : chaque séance terminée est enregistrée
//  localement sur l'iPhone (aucune donnée ne quitte l'appareil).
//

import Foundation
import SwiftData

/// Ressenti après une séance.
enum Ressenti: Int, Codable, CaseIterable, Identifiable {
    case difficile = 1
    case correct = 2
    case bien = 3

    var id: Int { rawValue }

    var emoji: String {
        switch self {
        case .difficile: "😮‍💨"
        case .correct: "🙂"
        case .bien: "😊"
        }
    }

    var libelle: String {
        switch self {
        case .difficile: "Difficile"
        case .correct: "Correct"
        case .bien: "Bien"
        }
    }
}

@Model
final class JournalSeance {
    var date: Date
    var titre: String
    var minutes: Int
    var exercicesFaits: Int
    var ressentiBrut: Int?

    var ressenti: Ressenti? {
        ressentiBrut.flatMap(Ressenti.init(rawValue:))
    }

    init(date: Date = .now, titre: String, minutes: Int, exercicesFaits: Int, ressenti: Ressenti?) {
        self.date = date
        self.titre = titre
        self.minutes = minutes
        self.exercicesFaits = exercicesFaits
        self.ressentiBrut = ressenti?.rawValue
    }
}

// MARK: - Statistiques

enum Statistiques {
    /// Jours consécutifs avec au moins une séance
    /// (la série tient si la séance du jour n'est pas encore faite).
    static func serieEnCours(_ journal: [JournalSeance]) -> Int {
        let calendrier = Calendar.current
        let jours = Set(journal.map { calendrier.startOfDay(for: $0.date) })
        var curseur = calendrier.startOfDay(for: .now)
        var serie = 0
        if !jours.contains(curseur) {
            curseur = calendrier.date(byAdding: .day, value: -1, to: curseur)!
        }
        while jours.contains(curseur) {
            serie += 1
            curseur = calendrier.date(byAdding: .day, value: -1, to: curseur)!
        }
        return serie
    }

    static func seancesSur7Jours(_ journal: [JournalSeance]) -> Int {
        let limite = Calendar.current.date(byAdding: .day, value: -6, to: Calendar.current.startOfDay(for: .now))!
        return journal.count(where: { $0.date >= limite })
    }

    static func minutesTotales(_ journal: [JournalSeance]) -> Int {
        journal.reduce(0) { $0 + $1.minutes }
    }

    static func seanceFaiteAujourdhui(_ journal: [JournalSeance]) -> Bool {
        journal.contains { Calendar.current.isDateInToday($0.date) }
    }
}
