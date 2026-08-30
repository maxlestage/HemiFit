//
//  Rappels.swift
//  HemiFit
//
//  Rappel quotidien par notification locale, pour garder le rythme.
//  Rien ne quitte l'iPhone.
//

import Foundation
import UserNotifications

@MainActor
@Observable
final class Rappels {
    static let identifiant = "hemifit.rappel.quotidien"

    var autorise = false

    func demanderAutorisation() async {
        let centre = UNUserNotificationCenter.current()
        autorise = (try? await centre.requestAuthorization(options: [.alert, .sound, .badge])) ?? false
    }

    /// Programme (ou reprogramme) le rappel quotidien à l'heure choisie.
    func programmer(heure: Int, minute: Int) async {
        let centre = UNUserNotificationCenter.current()
        centre.removePendingNotificationRequests(withIdentifiers: [Self.identifiant])

        let contenu = UNMutableNotificationContent()
        contenu.title = "C'est l'heure de HemiFit 💚"
        contenu.body = "Quelques minutes en douceur pour votre côté droit. Chaque petit mouvement compte."
        contenu.sound = .default

        var composants = DateComponents()
        composants.hour = heure
        composants.minute = minute

        let declencheur = UNCalendarNotificationTrigger(dateMatching: composants, repeats: true)
        let requete = UNNotificationRequest(
            identifier: Self.identifiant,
            content: contenu,
            trigger: declencheur
        )
        try? await centre.add(requete)
    }

    func annuler() {
        UNUserNotificationCenter.current()
            .removePendingNotificationRequests(withIdentifiers: [Self.identifiant])
    }
}
