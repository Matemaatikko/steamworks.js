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
    ensure_created: true,
    sort_method: 1,
    display_type: 1
}

const f = async () => {
    for (let i = 0; i < 5; i++) {
        const value = Math.floor(Math.random() * 1500)
        console.log(value)
        const result = await client?.leaderboard?.upload(leaderboard, value, [1, 2, 4, 5, 6, 7])
        console.log(result)
    }
}

const entries = async () => {
    const result = await client?.leaderboard?.getLeaderboard(leaderboard, 0, 5, 10)
    console.log(result)
}

entries()

// f()

