fetch("../data/build/public/data.json").then((res) => res.json()).then((data) => {
  console.log(data)
})