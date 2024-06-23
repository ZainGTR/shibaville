window.addEventListener("load", function () {
  // setup
  const canvas = document.getElementById("ville-canvas");
  const ctx = canvas.getContext("2d");
  canvas.width = 800;
  canvas.height = 600;

  function getArchId(address) {
    return archId;
  }

  class inputManager {}

  class VilleManager {}

  class Buildings {}

  class Hud {}

  class Player {
    constructor(game, wallet) {
      this.game = game;
      this.wallet = wallet;
      this.name = getArchId(wallet.address);
    }
  }

  class wallet {}

  class Game {}
});
