let currentUser = 'taveeh';
let currentPage = 0;
let field = 'basic';
let severityField = 'Error';
let typeField;
const insertData = (newBody, data) => {
    console.log(data);
    let result = data;
    console.log(result);
    for (let log of result) {
        let newRow = newBody.insertRow();
        if (result.indexOf(log) >= 4 * currentPage) {
            for (let index of ['id', 'log_type', 'severity', 'date', 'username', 'actual_log']) {
                let newCol = newRow.insertCell();
                let newText = document.createTextNode(log[index]);
                newCol.appendChild(newText);
            }
            newBody.append(newRow);
        }
        if (result.indexOf(log) >= 4 * currentPage + 3) {
            break;
        }
    }
}
const showLogReports = () => {
    let body = $('.logTable tbody').eq(0);
    let newBody = document.createElement('tbody');
    $.ajax({
        type: 'GET',
        url: "/allLogReports",
        success: (data) => {
            insertData(newBody, data)
        }
    })
    body.replaceWith(newBody);
}
//, user: currentUser
const showLogsByUser = () => {
    let body = $('.logTable tbody').eq(0);
    let newBody = document.createElement('tbody');
    $.ajax({
        type: 'GET',
        url: "/userLogReports",
        success: (data) => {
            insertData(newBody, data);
        }
    })
    body.replaceWith(newBody);
}

const showLogsBySeverity = (severity) => {
    let body = $('.logTable tbody').eq(0);
    let newBody = document.createElement('tbody');
    $.ajax({
        type: 'GET',
        url: "/severityLogReports",
        data: {severity: severity},
        success: (data) => {
            insertData(newBody, data);
        }
    })
    body.replaceWith(newBody);
}

const showLogsByType = (type) => {
    let body = $('.logTable tbody').eq(0);
    let newBody = document.createElement('tbody');
    $.ajax({
        type: 'GET',
        url: "/typeLogReports",
        data: {log_type: type},
        success: (data) => {
            insertData(newBody, data);
        }
    })
    body.replaceWith(newBody);
}
const getCorrectLogs = () => {
    switch (field) {
        case 'basic':
            showLogReports();
            break;
        case 'user':
            showLogsByUser();
            break;
        case 'severity':
            showLogsBySeverity(severityField);
            break;
        case 'type':
            showLogsByType(typeField);
            break;
    }
}
$(document).ready(() => {
    // let index = document.URL.indexOf('?');
    // currentUser = document.URL.substring(index + 'user='.length + 1).replaceAll(' ', '');
    showLogReports();

    $('#filterBySeverityButton').click(() => {
        currentPage = 0;
        field = 'severity';
        severityField = $('#severityInputFilter').val();
        getCorrectLogs();
    });

    $('#filterByUser').click(() => {
        currentPage = 0;
        field = 'user';
        getCorrectLogs();
    })

    $('#allLogsButton').click(() => {
        currentPage = 0;
        field = 'basic';
        getCorrectLogs();
    })

    $('#nextButton').click(() => {
        currentPage++;
        getCorrectLogs();
    })

    $('#previousButton').click(() => {
        if (currentPage > 0) {
            currentPage--;
        }
        getCorrectLogs();
    })

    $('#filterByTypeButton').click(() => {
       currentPage = 0;
       field = 'type';
       typeField = $('#typeInputFilter').val();
       getCorrectLogs();
    });

    $('#insertLogButton').click(() => {
        let type = $('#typeField').val();
        let severity = $('#severityField').val();
        let date = $('#dateField').val();
        let log = $('#logField').val();
        console.log([type, severity, currentUser, date, log]);
        $.ajax({
            type: "POST",
            url: "/addLog",
            data: {
                log_type: type,
                severity: severity,
                date: date,
                log: log
            },
            success: (data) => {
                if (data === 0) {
                    alert("Log could not be added");
                } else {
                    getCorrectLogs();
                }
            }
        })
    });

    $('#removeLogButton').click(() => {
        let id = Number($('#idField').val());
        $.ajax({
            type: "DELETE",
            url: "/deleteLog/" + id,
            success: (data) => {
                let res = JSON.parse(data);
                console.log("Remove: " + res);
                getCorrectLogs()
            }
        })
    })
});
