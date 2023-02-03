
/*
    SPDX-FileCopyrightText: %{CURRENT_YEAR} %{AUTHOR} <%{EMAIL}>
    SPDX-License-Identifier: LGPL-2.1-or-later
*/
import QtQuick 2.1
import QtQuick.Layouts 1.1
import QtQuick.Controls 2.5

import org.kde.plasma.core 2.0 as PlasmaCore
import org.kde.plasma.plasmoid 2.0
import org.kde.plasma.components 2.0 as PlasmaComponents

Item {
    Plasmoid.preferredRepresentation: Plasmoid.fullRepresentation
    Plasmoid.fullRepresentation: Item {
        property string lyric: ""
        Layout.preferredWidth: lyric_label.implicitWidth
        Layout.preferredHeight: lyric_label.implicitHeight
        //Layout.
        Label {
            id: lyric_label
            text: parent.lyric
            //horizontalAlignment: horizontalCenter
            //verticalAlignment: verticalCenter
            anchors.fill: parent
            verticalAlignment: Text.AlignVCenter //垂直居中，控件必须有height才可以使用
            horizontalAlignment: Text.AlignHCenter //水平居中，控件必须有width才可以使用
        }

        Timer {
            interval: 300
            onTriggered: {
                update()
            }
            repeat: true
            running: true
            triggeredOnStart: true
        }

        function update() {
            let xhr = new XMLHttpRequest()
            xhr.open("GET", "http://localhost:30123/lyric")
            xhr.send()
            xhr.onreadystatechange = function () {
                if (xhr.status == 200) {
                    lyric = xhr.responseText
                } else {
                    lyric = ""
                }
            }
        }
    }
}
