const { init } = require('../index.js')

const client = init(2916150)

console.log(client.localplayer.getName())

// const value = Math.floor(Math.random() * 1500)
// console.log(value)
// client?.leaderboard?.upload("Test", value, [1, 2, 4, 5, 6, 7]).then(e => console.log(e))

// for (let i = 0; i < 20; i++) {
//     const value = Math.floor(Math.random() * 1500)
//     console.log(value)
//     client?.leaderboard?.upload("Test", value, [1, 2, 4, 5, 6, 7]).then(e => console.log(e))
// }

const leaderboard = {
    name: "Test",
    ensureCreated: true,
    sortMethod: 1,
    displayType: 1
}

const leaderboard2 = {
    name: "Test 2",
    ensureCreated: true,
    sortMethod: 1,
    displayType: 1
}

const leaderboard3 = {
    name: "Test 3",
    ensureCreated: true,
    sortMethod: 0,
    displayType: 0
}


const f = async () => {
    for (let i = 0; i < 5; i++) {
        const value = Math.floor(Math.random() * 1500)
        console.log(value)
        const result = await client?.leaderboard?.upload(leaderboard, value, [1, 2, 4, 5, 6, 7])
        console.log(result)
    }
}

const insert = async (leaderboard) => {
    const value = Math.floor(Math.random() * 1500)
    const result = await client?.leaderboard?.upload(leaderboard, value, [1, 2, 4, 5, 6, 7])
    console.log("Upload", result)
}

const entries = async () => {
    const result = await client?.leaderboard?.getLeaderboardData(leaderboard, 0, 5, 10)
    console.log("Entries", result)
}

const personal = async (leaderboard) => {
    const result = await client?.leaderboard?.getUserLeaderboardData(leaderboard, 10)
    console.log("Personal", result)
}

//insert(leaderboard3)
personal(leaderboard3)

entries()

// personal(leaderboard3)

// f()

