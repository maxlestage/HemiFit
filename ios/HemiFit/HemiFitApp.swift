//
//  HemiFitApp.swift
//  HemiFit — Ma rééducation en douceur
//
//  Application personnelle de rééducation pour hémiparésie droite.
//  Tout est pensé pour être utilisé d'une seule main (la gauche) :
//  gros boutons, navigation basse, texte large.
//

import SwiftData
import SwiftUI

@main
struct HemiFitApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
        .modelContainer(for: JournalSeance.self)
    }
}
