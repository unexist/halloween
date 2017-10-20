$(document).ready(function() {
  var dots = $("#dots li");

  /* Add dots handler */
  dots.on("click", function() {
    var that = $(this);

    showPage(that.index());
  });

  /**
   * Handle select event of flickity
   **/

  function handleSelect()
  {
    var page = 0;

    $(".active").removeClass("active");

    var page_flkty = $page_carousel.data("flickity");
    var sign_flkty = $sign_carousel.data("flickity");

    var page_idx   = page_flkty.selectedIndex;
    var sign_idx   = sign_flkty.selectedIndex;

    page = page_idx + sign_idx;

    $(dots[page]).addClass("active");
  }


  /**
   * Select given page
   *
   * @param {Integer}  idx  Index of page
   **/

  function showPage(idx)
  {
    switch(idx)
      {
        case 0:
        case 1:
          $page_carousel.flickity("select", idx, false, false);
          $sign_carousel.flickity("select", 0,   false, false);
          break;

        case 2:
        case 3:
          $page_carousel.flickity("select", 1,       false, false);
          $sign_carousel.flickity("select", idx - 1, false, false);
      }
  }

  /**
   * Show toast message
   *
   * @param {String}  str  Message string to show
   ***/

  function showToast(str)
  {
    /* Insert flash element */
    var flash = $('<div class="flash" style="display: none;"></div>');
    var box   = $('<div class="box primary"></div>');
    var mesg  = $('<span></span>');

    mesg.append(str);
    box.append(mesg);
    flash.append(box);

    /* Click to hide box immediately */
    flash.click(function()
      {
        flash.remove();
      }
    );

    /* Find count of flash elements */
    var flashes = $("div.flash");

    $("#page").append(flash);

    if(0 < flashes.length)
      {
        flash.css("top", (parseInt(flash.css("top")) -
          flashes.length * (flash.outerHeight(true) + 10)) + "px");
      }

    /* Display message for some time */
    flash.fadeIn("slow").delay(5000).hide("slow", function()
      {
        var height = flash.outerHeight(true);

        flash.remove();

        var flashes = $("div.flash");

        if(0 < flashes.length)
          {
            for(var i = 0; i < flashes.length; i++)
              {
                var f = $(flashes[i]);

                f.css("top", (parseInt(f.css("top")) -
                  - height + 10) + "px");
              }
          }
      }
    );
  } 

  /* Init flickity */
  var $page_carousel = $("#page").flickity({
    pageDots:        false,
    prevNextButtons: false
  });

  var $sign_carousel = $("#box").flickity({
    pageDots:        false,
    prevNextButtons: false,
    cellAlign:       "left"
  });

  /* Add goTo handler */
  $(".goTo").click(function() {
    var that = $(this);

    showPage(parseInt(that.data("page")));
  });

  /* Add handler to update current page */
  $page_carousel.on("select.flickity", handleSelect);
  $sign_carousel.on("select.flickity", handleSelect);

  /* Handle submit */
  $("#submit").click(function() {
    var name   = $("#name").val();
    var brings = $("#brings").val();

    if("" != name)
      {
        /* Store data */
        $.ajax({
          type:     "POST",
          url:      "/ghost",
          data:     {
            name:   name,
            brings: brings
          },

          /* Success handler */
          success: function(data, status) {
            /* Insert data */
            $("#list table").append('<tr><td>' + name +
              '</td><td>' + brings + '</td></tr>');

            /* Clear */
            $("#name").val("");
            $("#brings").val("");

            /* Update UI */
            showPage(2);
            showToast("Yay");
          },

          /* Error handler */
          error: function(xhr, status, e) {
            showToast("Leider kaputt: " + status);
          }
        });

      }
    else showToast("Bitte alles ausfüllen");
  });
});
