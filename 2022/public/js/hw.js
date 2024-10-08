/*
 * @package hw
 *
 * @file jquery handler
 * @author Christoph Kappel <christoph@unexist.dev>
 * @version $Id: 2024/koppe.rb,v 0 1724858416.0-7200 unexist $
 *
 * This program can be distributed under the terms of the GNU GPLv3.
 * See the file COPYING for details.
 */

$(document).ready(function() {

    /**
     * Show toast message
     *
     * @param {String}    str    Message string to show
     ***/

    function showToast(str) {
        /* Insert flash element */
        var flash = $('<div class="flash" style="display: none;"></div>');
        var box     = $('<div class="box primary"></div>');
        var mesg    = $('<span></span>');

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

    /* Handle font */
    $("#font-switch").click(function() {
      $("body").css("font-family", '"Comic Sans MS", "Comic Sans", cursive');
    });

    /* Handle submit */
    $("#submit_vac, #submit_tst").click(function(e) {
        var name = $("#name").val();

        if ("" != name) {
            var isTested = "submit_tst" === e.target.id;
            var comment = $("#comment").val();

            /* Store data */
            $.ajax({
                type: "POST",
                url: "/ghost",
                data: {
                    name: name,
                    tested: isTested,
                    vacced: "submit_vac" === e.target.id,
                    comment: comment
                },

                /* Success handler */
                success: function (data, status) {
                    /* Insert data */
                    $("#list").append('<div class="' + (isTested ? "tested" : '')
                        + '">' + name + '</div>');

                    if ("" != comment) {
                        $('<tr><td class="comment" colspan="3"><span class="name">' + name
                            + '</span><span class="text">' + comment
                            + '</span></td></tr>').insertAfter($("#ghosts"));
                    }

                    /* Clear */
                    $("#name").val("");
                    $("#comment").val("");
                    $("#name").css("border-color", "white");

                    /* Update UI */
                    showToast("Yay");
                },

                /* Error handler */
                error: function (xhr, status, e) {
                    showToast("Das ging schief: " + status);
                }
            });

        }
        else {
            $("#name").css("border-color", "red");
            showToast("Fehlt da nicht irgendwas?");
        }
    });
});
